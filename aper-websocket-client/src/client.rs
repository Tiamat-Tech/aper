use crate::typed::TypedWebsocketConnection;
use anyhow::Result;
use aper::{
    connection::{ClientConnection, MessageToClient, MessageToServer},
    AperClient, Store,
};
use aper_stateroom::{IntentEvent, StateProgram};
use core::fmt::Debug;
use std::{
    rc::{Rc, Weak},
    sync::Mutex,
};

#[derive(Clone)]
pub struct AperWebSocketStateProgramClient<S>
where
    S: StateProgram,
{
    conn: Rc<Mutex<ClientConnection<S>>>,
}

pub struct IntentApplier<S>
where
    S: StateProgram,
{
    conn: Rc<Mutex<ClientConnection<S>>>,
}

impl<S> Clone for IntentApplier<S>
where
    S: StateProgram,
{
    fn clone(&self) -> Self {
        IntentApplier {
            conn: self.conn.clone(),
        }
    }
}

impl<S> IntentApplier<S>
where
    S: StateProgram,
{
    pub fn apply(&self, intent: S::WrappedIntent) {
        let mut conn = self.conn.lock().unwrap();

        let client = conn.client_id;
        let intent = IntentEvent {
            client,
            timestamp: chrono::Utc::now(),
            intent,
        };

        if let Err(err) = conn.apply(&intent) {
            tracing::error!("Error applying intent: {:?}", err);
        }
    }
}

impl<S> Debug for AperWebSocketStateProgramClient<S>
where
    S: StateProgram,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AperWebSocketStateProgramClient").finish()
    }
}

impl<S> AperWebSocketStateProgramClient<S>
where
    S: StateProgram,
{
    pub fn new(url: &str) -> Result<Self> {
        // callback is called when the state changes
        // need to create a connection
        // connection needs to be able to call the state and message callback

        // client message handler needs to have websocket connection; websocket
        // connection needs to be able to send messages to client

        let client = AperClient::<S>::new();

        let conn = Rc::new_cyclic(|c: &Weak<Mutex<ClientConnection<S>>>| {
            let d = c.clone();
            let socket_message_callback = move |message: MessageToClient| {
                let d = d.upgrade().unwrap();
                let mut conn = d.lock().unwrap();
                conn.receive(&message);
            };

            let wss_conn = TypedWebsocketConnection::new(url, socket_message_callback).unwrap();

            let message_callback = Box::new(move |message: MessageToServer| {
                wss_conn.send(&message);
            });

            Mutex::new(ClientConnection::new(client, message_callback))
        });

        Ok(AperWebSocketStateProgramClient { conn })
    }

    pub fn intent_applier(&self) -> IntentApplier<S> {
        IntentApplier {
            conn: self.conn.clone(),
        }
    }

    pub fn store(&self) -> Store {
        self.conn.lock().unwrap().store()
    }

    pub fn state(&self) -> S {
        let store = self.store();
        S::attach(store.handle())
    }

    pub fn apply(&self, intent: S::WrappedIntent) -> Result<(), S::Error> {
        let mut conn = self.conn.lock().unwrap();

        let client = conn.client_id;
        let intent = IntentEvent {
            client,
            timestamp: chrono::Utc::now(),
            intent,
        };

        conn.apply(&intent)
    }
}
