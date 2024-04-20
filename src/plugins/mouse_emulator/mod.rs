use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Res, Resource};
use bevy_persistent::Persistent;
use serde::{Deserialize, Serialize};

use crate::plugins::config_loader::{ConfigLoaderPlugin, Config};

pub struct MouseEmulatorPlugin;

impl Plugin for MouseEmulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConfigLoaderPlugin::<MouseEmulatorConfig>::default());
        app.add_systems(Update, test);
    }
}

#[derive(Resource, Serialize, Deserialize, Default, Debug)]
pub struct MouseEmulatorConfig {
    active: bool
}

impl Config for MouseEmulatorConfig {
    fn name() -> String {
        "mouse_emulator".to_string()
    }
}

fn test(config: Res<Persistent<MouseEmulatorConfig>>) {
    println!("{config:?}")
}
