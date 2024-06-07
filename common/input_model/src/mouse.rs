use serde::{Deserialize, Serialize};

use crate::input::AsModifier;
use crate::Modifier;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MouseInput {
    Button(ButtonInput),
    Move(Position),
    Wheel(Rotation),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ButtonInput {
    Pressed(Button),
    Released(Button),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Button {
    Left,
    Right,
    Middle,
    Unknown(u8),
}

impl AsModifier for Button {
    fn as_modifier(&self) -> Modifier {
        Modifier::Button(*self)
    }
}

impl From<&Button> for rdev::Button {
    fn from(value: &Button) -> Self {
        match value {
            Button::Left => rdev::Button::Left,
            Button::Right => rdev::Button::Right,
            Button::Middle => rdev::Button::Middle,
            Button::Unknown(button) => rdev::Button::Unknown(*button),
        }
    }
}

impl From<&rdev::Button> for Button {
    fn from(value: &rdev::Button) -> Self {
        match value {
            rdev::Button::Left => Button::Left,
            rdev::Button::Right => Button::Right,
            rdev::Button::Middle => Button::Middle,
            rdev::Button::Unknown(button) => Button::Unknown(*button),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Rotation {
    pub delta_x: i64,
    pub delta_y: i64,
}
