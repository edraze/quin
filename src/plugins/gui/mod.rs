// executor

use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Event, EventReader};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_egui::egui::{Color32, RichText};

const GUI_PLUGIN_NAME: &str = "gui";

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_event::<GuiEvent>();
        app.add_systems(Update, gui_handler);
    }

    fn name(&self) -> &str {
        GUI_PLUGIN_NAME
    }
}

#[derive(Event, Debug)]
pub enum GuiEvent {
    DrawLabel(String)
}

pub fn gui_handler(mut events: EventReader<GuiEvent>, mut contexts: EguiContexts) {
    let panel_frame = egui::Frame {
        fill: Color32::TRANSPARENT,
        ..Default::default()
    };
    for event in events.read() {
        match event {
            GuiEvent::DrawLabel(text) => {
                egui::CentralPanel::default()
                    .frame(panel_frame)
                    .show(contexts.ctx_mut(),
                          |ui| ui.label(RichText::new(text)
                              .strong()
                              .color(Color32::RED)));
            }
        }
    }
}
