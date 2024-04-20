use bevy::app::{App, Plugin, Update};
use bevy::prelude::{AssetServer, Commands, Component, Res};

pub struct ConfiguratorPlugin;

impl Plugin for ConfiguratorPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Component)]
pub struct Config;

fn load_configs (mut commands: Commands,
                 asset_server: Res<AssetServer>,) {
   asset_server.load_folder("").handles
}