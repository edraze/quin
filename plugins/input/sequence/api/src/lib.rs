use bevy::prelude::Event;
use serde::{Deserialize, Serialize};
use input_model::InputEvent;

#[derive(Event, Debug, Clone)]
pub struct Subscribe(pub Subscription);

#[derive(Event, Debug, Clone)]
pub struct Unsubscribe(pub Subscription);

#[derive(Debug, Clone)]
pub struct Subscription {
    pub subscriber: String,
    pub sequence: Sequence,
}

#[derive(Event, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Sequence {
    pub sequence: Vec<InputEvent>,
}

impl Sequence {
    pub fn new(input_sequence: Vec<InputEvent>) -> Self {
        Self {
            sequence: input_sequence
        }
    }
    pub fn len(&self) -> usize {
        self.sequence.len()
    }
}
