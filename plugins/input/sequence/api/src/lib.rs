use std::fmt::Debug;

use bevy::prelude::{Component, Event};
use serde::{Deserialize, Serialize};

use global_input_api::input::InputEvent;
use global_input_api::input_model::keyboard::{Key, KeyEvent};

#[derive(Component, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Sequence {
    pub input_events: Vec<InputEvent>,
}

impl Sequence {
    pub fn new(input_sequence: Vec<InputEvent>) -> Self {
        Self {
            input_events: input_sequence
        }
    }
    pub fn len(&self) -> usize {
        self.input_events.len()
    }
}

impl TryFrom<char> for Sequence {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let key = Key::try_from(value);
        Result::and_then(key, |key| {
            let input = vec![
                InputEvent::Keyboard(KeyEvent::Pressed(key)),
                InputEvent::Keyboard(KeyEvent::Released(key)),
            ];
            Ok(Sequence::new(input))
        })
    }
}

#[derive(Component, Clone)]
pub struct ToEvent<E: Event + Clone> {
    pub event: E,
}

impl<E: Event + Clone> ToEvent<E> {
    pub fn from_event(event: E) -> Self {
        Self {
            event
        }
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

#[derive(Event)]
pub struct ResetSequenceBuffer;
