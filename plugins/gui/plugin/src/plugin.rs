use bevy::app::{App, Plugin, Update};
use bevy_egui::EguiPlugin;

use ui_elements::draw_label_system;

const GUI_PLUGIN_NAME: &str = "gui";

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, draw_label_system);
    }

    fn name(&self) -> &str {
        GUI_PLUGIN_NAME
    }
}
