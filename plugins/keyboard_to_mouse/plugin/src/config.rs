use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use config_loader::Config;
use global_input_api::input::InputEvent;
use global_input_api::input_model::keyboard::{Key, KeyEvent};
use input_sequence_api::Sequence;
use crate::KEYBOARD_TO_MOUSE_PLUGIN_NAME;

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct KeyboardToMouseConfig {
    pub mouse_speed: i32,
    pub scroll_speed: i64,
    pub key_bindings: KeyboardToMouseKeyBindings,
}

impl Default for KeyboardToMouseConfig {
    fn default() -> Self {
        Self {
            mouse_speed: 10,
            scroll_speed: 1,
            key_bindings: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyboardToMouseKeyBindings {
    pub activate: Vec<Sequence>,
    pub deactivate: Vec<Sequence>,
    pub mouse_move_up: Vec<Sequence>,
    pub mouse_move_down: Vec<Sequence>,
    pub mouse_move_left: Vec<Sequence>,
    pub mouse_move_right: Vec<Sequence>,
    pub mouse_scroll_up: Vec<Sequence>,
    pub mouse_scroll_down: Vec<Sequence>,
    pub mouse_scroll_left: Vec<Sequence>,
    pub mouse_scroll_right: Vec<Sequence>,
    pub mouse_left_button_click: Vec<Sequence>,
    pub mouse_right_button_click: Vec<Sequence>,
    pub mouse_middle_button_click: Vec<Sequence>,
    pub mouse_drag_and_drop_activate: Vec<Sequence>,
    pub mouse_drag_and_drop_deactivate: Vec<Sequence>,
}

impl Default for KeyboardToMouseKeyBindings {
    fn default() -> Self {
        Self {
            activate: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::ControlRight))])],
            deactivate: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Released(Key::ControlRight))])],
            mouse_move_up: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyK))])],
            mouse_move_down: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyJ))])],
            mouse_move_left: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyH))])],
            mouse_move_right: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyL))])],
            mouse_scroll_up: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyU))])],
            mouse_scroll_down: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyD))])],
            mouse_scroll_left: vec![],
            mouse_scroll_right: vec![],
            mouse_left_button_click: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyI)),
                InputEvent::Keyboard(KeyEvent::Released(Key::KeyI)),
            ])],
            mouse_right_button_click: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyA)),
                InputEvent::Keyboard(KeyEvent::Released(Key::KeyA)),
            ])],
            mouse_middle_button_click: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyM)),
                InputEvent::Keyboard(KeyEvent::Released(Key::KeyM)),
            ])],
            mouse_drag_and_drop_activate: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyG)),
                InputEvent::Keyboard(KeyEvent::Released(Key::KeyG)),
            ])],
            mouse_drag_and_drop_deactivate: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyP)),
                InputEvent::Keyboard(KeyEvent::Released(Key::KeyP)),
            ])],
        }
    }
}

impl Config for KeyboardToMouseConfig {
    fn name() -> String {
        KEYBOARD_TO_MOUSE_PLUGIN_NAME.to_string()
    }
}
