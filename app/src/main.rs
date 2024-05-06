use bevy::prelude::App;
use crate::general::QuinPlugins;

pub(crate) mod config;
pub(crate) mod general;
pub(crate) mod systems;

fn main() {
    App::new()
        .add_plugins(QuinPlugins)
        .run();
}