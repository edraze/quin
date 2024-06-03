use bevy::app::{App, Plugin, Startup};

use global_input_plugin::GlobalInputPlugin;
use gui_plugin::GuiPlugin;
use input_sequence_plugin::InputSequencePlugin;
use keyboard_to_mouse_plugin::KeyboardToMousePlugin;
use mouse_output_plugin::MouseOutputPlugin;
use navigation_grid::NavigationGridPlugin;
use overlay_plugin::OverlayPlugin;
#[cfg(target_os = "windows")]
use tiling_window_manager::TilingWindowManagerPlugin;
use tray_plugin::TrayPlugin;

use crate::config::GeneralConfig;
use crate::systems::auto_startup_system;

pub const GENERAL_PLUGIN_NAME: &str = "general";

pub struct QuinPlugins;

impl Plugin for QuinPlugins {
    fn build(&self, app: &mut App) {
        let config = config_loader::load::<GeneralConfig>();
        
        app.insert_resource(config.clone());
        app.add_systems(Startup, auto_startup_system);

        add_plugin_if_enabled(app, &config, OverlayPlugin);
        add_plugin_if_enabled(app, &config, TrayPlugin);
        add_plugin_if_enabled(app, &config, GlobalInputPlugin);
        add_plugin_if_enabled(app, &config, InputSequencePlugin);
        add_plugin_if_enabled(app, &config, GuiPlugin);
        add_plugin_if_enabled(app, &config, KeyboardToMousePlugin);
        add_plugin_if_enabled(app, &config, NavigationGridPlugin);
        add_plugin_if_enabled(app, &config, MouseOutputPlugin);
        #[cfg(target_os = "windows")]
        add_plugin_if_enabled(app, &config, TilingWindowManagerPlugin);
    }
    fn name(&self) -> &str {
        GENERAL_PLUGIN_NAME
    }
}


fn add_plugin_if_enabled(app: &mut App, config: &GeneralConfig, plugin: impl Plugin) {
    if config.is_enabled(&plugin) {
        app.add_plugins(plugin);
    }
}
