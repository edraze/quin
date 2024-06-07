use rdev::EventType;
use serde::{Deserialize, Serialize};

use crate::{Button, ButtonInput, Key, Position, Rotation};
use crate::keyboard::KeyboardInput;
use crate::mouse::MouseInput;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Sequence {
    pub input_events: Vec<Input>,
}

impl Sequence {
    pub fn new(input_sequence: Vec<Input>) -> Self {
        Self {
            input_events: input_sequence
        }
    }
    pub fn length(&self) -> usize {
        self.input_events.len()
    }
}

impl From<Input> for Sequence {
    fn from(value: Input) -> Self {
        Sequence::new(vec![value])
    }
}

impl From<Input> for Vec<Sequence> {
    fn from(value: Input) -> Self {
        vec![value.into()]
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Input {
    Device(DeviceInput),
    Modified(Modified),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum DeviceInput {
    Keyboard(KeyboardInput),
    Mouse(MouseInput),
}

impl From<&rdev::Event> for DeviceInput {
    fn from(value: &rdev::Event) -> Self {
        let event = value.event_type;
        match &event {
            EventType::KeyPress(key) => DeviceInput::Keyboard(KeyboardInput::Pressed(key.into())),
            EventType::KeyRelease(key) => DeviceInput::Keyboard(KeyboardInput::Released(key.into())),
            EventType::ButtonPress(button) => DeviceInput::Mouse(MouseInput::Button(ButtonInput::Pressed(button.into()))),
            EventType::ButtonRelease(button) => DeviceInput::Mouse(MouseInput::Button(ButtonInput::Released(button.into()))),
            EventType::MouseMove { x, y } => DeviceInput::Mouse(MouseInput::Move(Position { x: *x, y: *y })),
            EventType::Wheel { delta_x, delta_y } => DeviceInput::Mouse(MouseInput::Wheel(Rotation { delta_x: *delta_x, delta_y: *delta_y })),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Modified {
    pub modifiers: Vec<Modifier>,
    pub input: Box<Input>,
}

impl From<Modified> for Input {
    fn from(value: Modified) -> Self {
        Input::Modified(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub enum Modifier {
    Key(Key),
    Button(Button),
}

pub trait AsModifier {
    fn as_modifier(&self) -> Modifier;
}

impl<M> From<M> for Modifier
    where M: AsModifier {
    fn from(value: M) -> Self {
        value.as_modifier()
    }
}