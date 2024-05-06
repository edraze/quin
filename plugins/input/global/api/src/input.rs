use bevy::prelude::Event;
use rdev::EventType;
use serde::{Deserialize, Serialize};
use input_model::keyboard::KeyEvent;
use input_model::mouse::ButtonEvent;

#[derive(Event, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum InputEvent {
    // todo remove indirection if it possible
    Keyboard(KeyEvent),
    MouseButton(ButtonEvent),
    MouseMove {
        x: f64,
        y: f64,
    },
    Wheel {
        delta_x: i64,
        delta_y: i64,
    },
}

impl From<&rdev::Event> for InputEvent {
    fn from(value: &rdev::Event) -> Self {
        let event = value.event_type;
        match &event {
            EventType::KeyPress(key) => InputEvent::Keyboard(KeyEvent::Pressed(key.into())),
            EventType::KeyRelease(key) => InputEvent::Keyboard(KeyEvent::Released(key.into())),
            EventType::ButtonPress(button) => InputEvent::MouseButton(ButtonEvent::Pressed(button.into())),
            EventType::ButtonRelease(button) => InputEvent::MouseButton(ButtonEvent::Released(button.into())),
            EventType::MouseMove { x, y } => InputEvent::MouseMove { x: *x, y: *y },
            EventType::Wheel { delta_x, delta_y } => InputEvent::Wheel { delta_x: *delta_x, delta_y: *delta_y },
        }
    }
}
