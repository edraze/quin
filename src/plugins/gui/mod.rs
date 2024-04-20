// executor

use bevy::prelude::{Event, EventReader};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Color32, RichText};

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
