use std::fmt::Debug;

use bevy::prelude::{Component};
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