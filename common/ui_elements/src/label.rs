use bevy::prelude::{Component, Query};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Color32, FontId, Pos2, Shape, Stroke, Ui};
use bevy_egui::egui::epaint::RectShape;

#[derive(Component)]
pub struct UiLabel {
    pub visible: bool,
    pub position: Pos2,
    pub text: String,
}

impl UiLabel {
    pub fn new(visible: bool, x: f32, y: f32, text: &str) -> Self {
        Self {
            visible,
            position: (x, y).into(),
            text: text.to_string(),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }

    fn draw(&self, ui: &mut Ui) {
        if self.visible {
            let painter = ui.painter();
            let position = self.position;
            let text = self.text.clone();

            painter.add(egui::epaint::CircleShape::filled(position, -1., Color32::RED));

            let galley = painter.layout(
                text,
                FontId::proportional(10.),
                Color32::YELLOW,
                f32::INFINITY,
            );
            let text_shape = egui::epaint::TextShape::new(position, galley, Color32::RED);

            let text_background = Shape::Rect(RectShape::new(
                text_shape.visual_bounding_rect().expand(1.),
                2.,
                Color32::from_rgb(30, 31, 34),
                Stroke::NONE,
            ));
            painter.add(text_background);
            painter.add(text_shape);
        }
    }
}

pub fn draw_label_system(labels: Query<&mut UiLabel>, mut contexts: EguiContexts) {
    let panel_frame = egui::Frame {
        fill: Color32::TRANSPARENT,
        rounding: 0.0.into(),
        stroke: Stroke::NONE,
        outer_margin: 0.0.into(),
        ..Default::default()
    };

    let context = contexts.ctx_mut();
    egui::CentralPanel::default().frame(panel_frame).show(context, |ui|
        labels.iter()
            .for_each(|label| label.draw(ui)));
}