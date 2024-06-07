use std::fmt::Debug;

use bevy::prelude::{Component, Event};
use serde::{Deserialize, Serialize};

use global_input_api::input_model::Sequence;

#[derive(Component, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SequenceComponent(pub Sequence);

impl From<Sequence> for SequenceComponent {
    fn from(value: Sequence) -> Self {
        SequenceComponent(value)
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
    pub sequences: Vec<SequenceComponent>,
    pub event: E,
}

impl<E: Event + Clone> From<(Vec<Sequence>, E)> for SequencesToEvent<E> {
    fn from((sequences, event): (Vec<Sequence>, E)) -> Self {
        let sequences = sequences.into_iter()
            .map(|sequence| sequence.into())
            .collect();
        Self {
            sequences,
            event,
        }
    }
}

impl<E: Event + Clone> From<(Sequence, E)> for SequencesToEvent<E> {
    fn from((sequence, event): (Sequence, E)) -> Self {
        let sequences = vec![sequence.into()];
        Self {
            sequences,
            event,
        }
    }
}

#[derive(Event)]
pub struct ResetSequenceBuffer;
