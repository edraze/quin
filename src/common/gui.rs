use egui_backend::egui;
use egui_backend::egui::{Color32, Pos2};

pub fn init_frame(ctx: &egui::Context) {
    let panel_frame = egui::Frame {
        fill: Color32::TRANSPARENT,
        rounding: 0.0.into(),
        stroke: egui::Stroke::none(),
        outer_margin: 0.0.into(),
        ..Default::default()
    };
    egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        ui.painter().add(egui::epaint::CircleShape::filled(Pos2::new(0.0, 0.0), 0.6, Color32::BLACK));
    });
}
