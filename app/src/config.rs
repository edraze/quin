use bevy::app::Plugin;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use config_loader::Config;
use global_input_plugin::GlobalInputPlugin;
use gui_plugin::GuiPlugin;
use input_sequence_plugin::InputSequencePlugin;
use keyboard_to_mouse_plugin::KeyboardToMousePlugin;
use mouse_output_plugin::MouseOutputPlugin;
use navigation_grid::NavigationGridPlugin;
use overlay_plugin::OverlayPlugin;
use tray_plugin::TrayPlugin;
use crate::general::GENERAL_PLUGIN_NAME;

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct GeneralConfig {
    pub auto_startup: bool,
    pub plugins: Vec<String>,
}

impl GeneralConfig {
    pub fn is_enabled(&self, plugin: &impl Plugin) -> bool {
        let plugin_name = plugin.name().to_string();
        self.plugins.contains(&plugin_name)
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            auto_startup: false,
            plugins: vec![
                TrayPlugin.name().to_string(),
                GlobalInputPlugin.name().to_string(),
                InputSequencePlugin.name().to_string(),
                OverlayPlugin.name().to_string(),
                GuiPlugin.name().to_string(),
                KeyboardToMousePlugin.name().to_string(),
                NavigationGridPlugin.name().to_string(),
                MouseOutputPlugin.name().to_string(),
            ],
        }
    }
}

impl Config for GeneralConfig {
    fn name() -> String {
        GENERAL_PLUGIN_NAME.to_string()
    }
}
