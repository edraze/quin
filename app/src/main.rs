use bevy::prelude::App;

use crate::general::QuinPlugins;

mod general;

fn main() {
    App::new()
        .add_plugins(QuinPlugins)
        .run();
}