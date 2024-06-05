use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use config_loader::Config;
use global_input_api::input_model::definition::{P, R};
use global_input_api::input_model::Key;
use global_input_api::input_model::Key::{A, C, ControlRight, D, G, H, I, J, K, L, U};
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
            activate: vec![Sequence::new(vec![P(ControlRight).into()])],
            deactivate: vec![Sequence::new(vec![R(ControlRight).into()])],
            mouse_move_up: vec![Sequence::new(vec![P(K).into()])],
            mouse_move_down: vec![Sequence::new(vec![P(J).into()])],
            mouse_move_left: vec![Sequence::new(vec![P(H).into()])],
            mouse_move_right: vec![Sequence::new(vec![P(L).into()])],
            mouse_scroll_up: vec![Sequence::new(vec![P(U).into()])],
            mouse_scroll_down: vec![Sequence::new(vec![P(D).into()])],
            mouse_scroll_left: vec![],
            mouse_scroll_right: vec![],
            mouse_left_button_click: vec![Sequence::new(vec![
                P(I).into(),
                R(I).into(),
            ])],
            mouse_right_button_click: vec![Sequence::new(vec![
                P(A).into(),
                R(A).into(),
            ])],
            mouse_middle_button_click: vec![Sequence::new(vec![
                P(C).into(),
                R(C).into(),
            ])],
            mouse_drag_and_drop_activate: vec![Sequence::new(vec![
                P(G).into(),
                R(G).into(),
            ])],
            mouse_drag_and_drop_deactivate: vec![Sequence::new(vec![
                P(Key::P).into(),
                R(Key::P).into(),
            ])],
        }
    }
}

impl Config for KeyboardToMouseConfig {
    fn name() -> String {
        KEYBOARD_TO_MOUSE_PLUGIN_NAME.to_string()
    }
}
