use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Res, Resource};
use bevy_persistent::Persistent;
use serde::{Deserialize, Serialize};

use crate::plugins::config_loader::{ConfigLoaderPlugin, Config};

const MOUSE_EMULATOR_PLUGIN_NAME: &str = "mouse_emulator";

pub struct MouseEmulatorPlugin;

impl Plugin for MouseEmulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConfigLoaderPlugin::<MouseEmulatorConfig>::default());
        app.add_systems(Update, test);
    }

    fn name(&self) -> &str {
        MOUSE_EMULATOR_PLUGIN_NAME
    }
}

#[derive(Resource, Serialize, Deserialize, Default, Debug)]
pub struct MouseEmulatorConfig {
    key_binding: MouseEmulatorKeyBindings,
    scroll_speed: i64, // todo i8
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MouseEmulatorKeyBindings {
    // activate: , // Vec<Vec<InputEvent>>
    // deactivate: ,
    // mouse_move_up: ,
    // mouse_move_down: ,
    // mouse_move_left: ,
    // mouse_move_right: ,
    // mouse_left_button_click: ,
    // mouse_right_button_click:,
    // mouse_middle_button_click:,
    // mouse_scroll_up:,
    // mouse_scroll_down: ,
    // mouse_drag_and_drop_activate: ,
    // mouse_drag_and_drop_deactivate: ,
}

impl Config for MouseEmulatorConfig {
    fn name() -> String {
        MOUSE_EMULATOR_PLUGIN_NAME.to_string()
    }
}

fn test(config: Res<Persistent<MouseEmulatorConfig>>) {
    println!("{config:?}")
}
