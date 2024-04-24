use bevy::prelude::App;

use crate::general::QuinPlugins;

mod general;

fn main() {
    // todo move to plugin (https://bevy-cheatbook.github.io/programming/non-send.html)
    let _tray_icon = tray_plugin::build_tray(&["Exit".to_string()]);

    App::new()
        .add_plugins(QuinPlugins)
        .run();
}