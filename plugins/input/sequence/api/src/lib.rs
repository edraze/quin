use std::fmt::Debug;

use bevy::prelude::{Component, Event};
use serde::{Deserialize, Serialize};

use global_input_api::input::InputEvent;

#[derive(Component, Serialize, Deserialize, Debug, PartialEq, Clone)]
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

pub struct SequencesToEvent<E: Event + Clone> {
    pub sequences: Vec<Sequence>,
    pub event: E,
}

impl<E: Event + Clone> From<(Vec<Sequence>, E)> for SequencesToEvent<E> {
    fn from((sequences, event): (Vec<Sequence>, E)) -> Self {
        Self {
            sequences,
            event,
        }
    }
}

impl<E: Event + Clone> From<(Sequence, E)> for SequencesToEvent<E> {
    fn from((sequence, event): (Sequence, E)) -> Self {
        let sequences = vec![sequence];
        Self {
            sequences,
            event,
        }
    }
}
