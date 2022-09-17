use crate::{StateProgram, StateProgramMessage, TransitionEvent};
use aper::sync::{client::StateClient, messages::MessageToServer};
use chrono::{DateTime, Duration, Utc};
use stateroom::ClientId;
use std::rc::Rc;

#[derive(Debug)]
pub struct InnerState<S: StateProgram> {
    client: StateClient<S>,
    pub client_id: ClientId,
    pub server_time_delta: Duration,
}

impl<S: StateProgram> InnerState<S> {
    fn current_server_time(&self) -> DateTime<Utc> {
        Utc::now()
            .checked_sub_signed(self.server_time_delta)
            .unwrap()
    }

    pub fn state(&self) -> Rc<S> {
        self.client.state()
    }

    fn wrap_transition(&self, transition: S::T) -> TransitionEvent<S::T> {
        let timestamp = self.current_server_time();

        TransitionEvent {
            client: Some(self.client_id),
            timestamp,
            transition,
        }
    }
}

pub struct StateProgramClient<S: StateProgram> {
    inner_state: Option<InnerState<S>>,
}

impl<S: StateProgram> Default for StateProgramClient<S> {
    fn default() -> Self {
        StateProgramClient { inner_state: None }
    }
}

impl<S: StateProgram> StateProgramClient<S> {
    pub fn receive_message_from_server(
        &mut self,
        message: StateProgramMessage<S>,
    ) -> Option<MessageToServer<S>> {
        match (message, &mut self.inner_state) {
            (
                StateProgramMessage::InitialState {
                    timestamp,
                    client_id,
                    state,
                    version,
                },
                None,
            ) => {
                let client = StateClient::new(state, version);
                let server_time_delta = Utc::now().signed_duration_since(timestamp);
                self.inner_state.replace(InnerState {
                    client,
                    client_id,
                    server_time_delta,
                });
                None
            }
            (StateProgramMessage::Message { message, timestamp }, Some(inner_state)) => {
                if let Some(response) = inner_state.client.receive_message_from_server(message) {
                    return Some(response);
                }
                let server_time_delta = Utc::now().signed_duration_since(timestamp);
                inner_state.server_time_delta = server_time_delta;
                None
            }
            (message, _) => panic!(
                "Received message {:?} while in state {:?}.",
                message, self.inner_state
            ),
        }
    }

    pub fn push_transition(&mut self, transition: S::T) -> Result<MessageToServer<S>, S::Conflict> {
        if let Some(inner_state) = &mut self.inner_state {
            let transition = inner_state.wrap_transition(transition);
            inner_state.client.push_transition(transition)
        } else {
            panic!();
        }
    }

    pub fn state(&self) -> Option<&InnerState<S>> {
        if let Some(inner_state) = &self.inner_state {
            Some(inner_state)
        } else {
            None
        }
    }
}
