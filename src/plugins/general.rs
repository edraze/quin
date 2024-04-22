use bevy::app::{App, Plugin};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use crate::plugins::config_loader;
use crate::plugins::config_loader::Config;
use crate::plugins::exit::ExitPlugin;
use crate::plugins::global_input::GlobalInputPlugin;
use crate::plugins::gui::GuiPlugin;
use crate::plugins::input_sequence::InputSequencePlugin;
use crate::plugins::input_to_gui::InputToGuiPlugin;
use crate::plugins::mouse_emulator::MouseEmulatorPlugin;
use crate::plugins::overlay::OverlayPlugin;
use crate::plugins::sequence_to_log::SequenceToLogPlugin;
use crate::plugins::tray::TrayPlugin;

const GENERAL_PLUGIN_NAME: &str = "general";

pub struct QuinPlugins;

impl Plugin for QuinPlugins {
    fn build(&self, app: &mut App) {
        let config = config_loader::load_config::<GeneralConfig>();

        // todo only for dev purpose
        add_plugin_if_enabled(app, &config, InputToGuiPlugin);
        add_plugin_if_enabled(app, &config, SequenceToLogPlugin);
        
        add_plugin_if_enabled(app, &config, TrayPlugin);
        add_plugin_if_enabled(app, &config, ExitPlugin);
        add_plugin_if_enabled(app, &config, GlobalInputPlugin);
        add_plugin_if_enabled(app, &config, InputSequencePlugin);
        add_plugin_if_enabled(app, &config, OverlayPlugin);
        add_plugin_if_enabled(app, &config, GuiPlugin);
        add_plugin_if_enabled(app, &config, MouseEmulatorPlugin);
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
                ExitPlugin.name().to_string(),
                GlobalInputPlugin.name().to_string(),
                InputSequencePlugin.name().to_string(),
                OverlayPlugin.name().to_string(),
                GuiPlugin.name().to_string(),
                MouseEmulatorPlugin.name().to_string(),
                
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