use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ButtonEvent {
    Pressed(Button),
    Released(Button),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum Button {
    Left,
    Right,
    Middle,
    Unknown(u8),
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