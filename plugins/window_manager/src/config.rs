use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use config_loader::Config;
use global_input_api::input::InputEvent;
use global_input_api::input_model::keyboard::{Key, KeyEvent};
use input_sequence_api::Sequence;

use crate::TILING_WINDOW_MANAGER_PLUGIN_NAME;

#[derive(Resource, Serialize, Deserialize, Debug, Clone, Default)]
pub struct TilingWindowManagerConfig {
    pub key_bindings: TilingWindowManagerBindings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilingWindowManagerBindings {
    pub focus_left: Vec<Sequence>,
    pub focus_right: Vec<Sequence>,
    pub focus_up: Vec<Sequence>,
    pub focus_down: Vec<Sequence>,
    pub move_left: Vec<Sequence>,
    pub move_right: Vec<Sequence>,
    pub move_up: Vec<Sequence>,
    pub move_down: Vec<Sequence>,
    pub stack_left: Vec<Sequence>,
    pub stack_right: Vec<Sequence>,
    pub stack_up: Vec<Sequence>,
    pub stack_down: Vec<Sequence>,
    pub unstack: Vec<Sequence>,
    pub toggle_maximize: Vec<Sequence>,
    pub toggle_monocle: Vec<Sequence>,
    pub toggle_float: Vec<Sequence>,
    pub minimize: Vec<Sequence>,
    pub close: Vec<Sequence>,
}

impl Default for TilingWindowManagerBindings {
    fn default() -> Self {
        Self {
            focus_left: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyJ))])],
            focus_right: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::SemiColon))])],
            focus_up: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyL))])],
            focus_down: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyK))])],
            move_left: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::ShiftLeft)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyJ))])],
            move_right: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::ShiftLeft)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::SemiColon))])],
            move_up: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::ShiftLeft)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyL))])],
            move_down: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::ShiftLeft)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyK))])],
            stack_left: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::LeftArrow))])],
            stack_right: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::RightArrow))])],
            stack_up: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::UpArrow))])],
            stack_down: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::DownArrow))])],
            unstack: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Escape))])],
            toggle_maximize: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyO))])],
            toggle_monocle: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT))])],
            toggle_float: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyF))])],
            minimize: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyM))])],
            close: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyX))])],
        }
    }
}

impl Config for TilingWindowManagerConfig {
    fn name() -> String {
        TILING_WINDOW_MANAGER_PLUGIN_NAME.to_string()
    }
}
