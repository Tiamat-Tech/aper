use aper::{StateMachine, Transition};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Counter {value: i64}

#[derive(Transition, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CounterTransition {
    Add(i64),
    Subtract(i64),
    Reset,
}

impl StateMachine for Counter {
    type Transition = CounterTransition;

    fn apply(&mut self, event: CounterTransition) {
        match event {
            CounterTransition::Add(i) => {
                self.value += i;
            }
            CounterTransition::Subtract(i) => {
                self.value -= i;
            }
            CounterTransition::Reset => {
                self.value = 0;
            }
        }
    }
}
