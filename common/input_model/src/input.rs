use rdev::EventType;
use serde::{Deserialize, Serialize};

use crate::{Button, ButtonInput, Key, Position, Rotation};
use crate::keyboard::KeyboardInput;
use crate::mouse::MouseInput;

// [...].into()
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Sequence {
    input_events: Vec<Input>,
}

// Input::Device(DeviceInput::Keyboard(KeyboardEvent::Pressed(Key::KeyT)) 
// | Device(Keyboard(Pressed(KeyT)) 
// | Pressed(KeyT).into()

// Input::Modified(Modified::new([DeviceInput::Keyboard(KeyboardEvent::Pressed(Key::AltRight))], DeviceInput::Keyboard(KeyboardEvent::Pressed(Key::KeyT)))) 
// | Modified(Modified::new([Keyboard(Pressed(AltRight))], Keyboard(Pressed(KeyT)))) 
// | ([Pressed(AltRight)], Pressed(KeyT)).into()

// Input::Idle(Duration::from_secs(10));
// | 10.into()
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Input {
    Device(DeviceInput),
    Modified(Modified),
    // Idle(Duration),
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub enum Modifier {
    Key(Key),
    Button(Button),
}

impl From<Key> for Modifier {
    fn from(value: Key) -> Self {
        Modifier::Key(value)
    }
}

impl From<Button> for Modifier {
    fn from(value: Button) -> Self {
        Modifier::Button(value)
    }
}

impl From<Key> for Vec<Modifier> {
    fn from(value: Key) -> Self {
        vec![Modifier::Key(value)]
    }
}

impl From<Button> for Vec<Modifier> {
    fn from(value: Button) -> Self {
        vec![Modifier::Button(value)]
    }
}

