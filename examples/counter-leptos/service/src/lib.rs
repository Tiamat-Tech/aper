use aper_stateroom::{AperStateroomService, StateMachineContainerProgram};
use counter_common::Counter;
use stateroom_wasm::stateroom_wasm;

#[stateroom_wasm]
type CounterService = AperStateroomService<StateMachineContainerProgram<Counter>>;
