use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

use input_model::{Button, Input, Key};
use input_model::definition::{P, R};

#[derive(Event, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct InputEvent(pub Input);

impl From<P<Key>> for InputEvent {
    fn from(value: P<Key>) -> Self {
        InputEvent(value.into())
    }
}

impl From<P<Button>> for InputEvent {
    fn from(value: P<Button>) -> Self {
        InputEvent(value.into())
    }
}

impl From<R<Key>> for InputEvent {
    fn from(value: R<Key>) -> Self {
        InputEvent(value.into())
    }
}

impl From<R<Button>> for InputEvent {
    fn from(value: R<Button>) -> Self {
        InputEvent(value.into())
    }
}

impl From<(Key, Input)> for InputEvent {
    fn from(value: (Key, Input)) -> Self {
        InputEvent(value.into())
    }
}

impl From<(&[Key], Input)> for InputEvent {
    fn from(value: (&[Key], Input)) -> Self {
        InputEvent(value.into())
    }
}

impl From<(Button, Input)> for InputEvent {
    fn from(value: (Button, Input)) -> Self {
        InputEvent(value.into())
    }
}

impl From<(&[Button], Input)> for InputEvent {
    fn from(value: (&[Button], Input)) -> Self {
        InputEvent(value.into())
    }
}