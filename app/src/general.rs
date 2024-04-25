use bevy::app::{App, Plugin};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use config_loader::Config;
use global_input_plugin::GlobalInputPlugin;
use input_to_gui_plugin::InputToGuiPlugin;
use keyboard_to_mouse_plugin::KeyboardToMousePlugin;
use mouse_output_plugin::MouseOutputPlugin;
use overlay_plugin::OverlayPlugin;
use sequence_to_log_plugin::SequenceToLogPlugin;
use tray_plugin::TrayPlugin;
use gui_plugin::GuiPlugin;
use input_sequence_plugin::InputSequencePlugin;

const GENERAL_PLUGIN_NAME: &str = "general";

pub struct QuinPlugins;

impl Plugin for QuinPlugins {
    fn build(&self, app: &mut App) {
        let config = config_loader::load_config::<GeneralConfig>();

        add_plugin_if_enabled(app, &config, TrayPlugin);
        add_plugin_if_enabled(app, &config, GlobalInputPlugin);
        add_plugin_if_enabled(app, &config, InputSequencePlugin);
        add_plugin_if_enabled(app, &config, OverlayPlugin);
        add_plugin_if_enabled(app, &config, GuiPlugin);
        add_plugin_if_enabled(app, &config, KeyboardToMousePlugin);
        add_plugin_if_enabled(app, &config, MouseOutputPlugin);
        
        // todo only for dev purpose
        add_plugin_if_enabled(app, &config, InputToGuiPlugin);
        add_plugin_if_enabled(app, &config, SequenceToLogPlugin);
    }
    fn name(&self) -> &str {
        GENERAL_PLUGIN_NAME
    }
}

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct GeneralConfig {
    // todo implement auto startup
    auto_startup: bool,
    plugins: Vec<String>,
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
                MouseOutputPlugin.name().to_string(),
                
                InputToGuiPlugin.name().to_string(),
                SequenceToLogPlugin.name().to_string(),
            ],
        }
    }
}

impl Config for GeneralConfig {
    fn name() -> String {
        GENERAL_PLUGIN_NAME.to_string()
    }
}

fn add_plugin_if_enabled(app: &mut App, config: &GeneralConfig, plugin: impl Plugin) {
    if config.is_enabled(&plugin) {
        app.add_plugins(plugin);
    }
}
