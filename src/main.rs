use bevy::prelude::*;

use crate::plugins::general::QuinPlugins;

mod plugins;
mod systems;

fn main() {
    // todo move to plugin (https://bevy-cheatbook.github.io/programming/non-send.html)
    let _tray_icon = plugins::tray::build_tray(&["Exit".to_string()]);

    App::new()
        .add_plugins(QuinPlugins)
        .run();
}
