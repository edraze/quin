use bevy::prelude::Event;

use crate::plugins::global_input::events::InputEvent;

#[derive(Event, Debug, Clone)]
pub struct SubscribeToSequence(pub Subscription);

#[derive(Event, Debug, Clone)]
pub struct UnsubscribeToSequence(pub Subscription);

#[derive(Debug, Clone)]
pub struct Subscription {
    pub subscriber: String,
    pub sequence: Sequence,
}

#[derive(Event, Debug, PartialEq, Clone)]
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