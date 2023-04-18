use std::collections::HashMap;
use egui_backend::egui;
use egui_backend::egui::{Color32, Context, FontFamily, FontId, Pos2, Ui};
use rdev::EventType;
use crate::common;
use crate::core::{Bind, Binding, Draw, Handler, Label, State};

const GM_ACTIVATE: &str = "gm_activate";
const POINT_KEY_LETTERS: [&str; 21] = ["a", "b", "c", "d", "e", "f", "g", /*"h",*/ "i", /*"j", "k", "l",*/ "m", "n", "o", "p", "q", "r", "s", "t", "u", /*"v",*/ "w", "x", "y", "z"]; // todo replace by range

pub struct GridModeHandler {
    is_mode_active: bool,
    points: HashMap<String, Point>,
}

impl Default for GridModeHandler {
    fn default() -> Self {
        let key_to_point = generate_key_to_points();
        GridModeHandler { is_mode_active: false, points: key_to_point }
    }
}

#[derive(Default, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Bind for GridModeHandler {
    fn get_bindings(&self) -> Vec<Binding> {
        let mut bindings: Vec<Binding> = self.points.keys()
            .map(|label| {
                let default_input = common::keys::string_to_buffer(label);
                Binding { label: label.to_string(), default_input }
            })
            .collect();
        bindings.push(Binding { label: GM_ACTIVATE.to_string(), default_input: "RAltRight".to_string() });
        bindings
    }
}

impl Draw for GridModeHandler {
    fn draw(&self, gui_ctx: &Context) {
        if self.is_mode_active {
            self.draw_grid(gui_ctx);
        } else {
            common::gui::init_frame(gui_ctx);
        }
    }
}

impl Handler for GridModeHandler {
    fn execute(&mut self, label: &Label, _: &mut State) {
        if let Label::Keys(label) = label {
            if label.eq(GM_ACTIVATE) {
                self.is_mode_active = !self.is_mode_active;
            }
            if self.is_mode_active {
                if let Some(Point { x, y }) = self.points.get(label) {
                    rdev::simulate(&EventType::MouseMove { x: *x as f64, y: *y as f64 }).unwrap();
                }
            }
        }
    }
}

impl GridModeHandler {
    fn draw_grid(&self, gui_ctx: &Context) {
        let panel_frame = egui::Frame {
            fill: Color32::TRANSPARENT,
            rounding: 0.0.into(),
            stroke: egui::Stroke::none(),
            outer_margin: 0.0.into(),
            ..Default::default()
        };

        egui::CentralPanel::default().frame(panel_frame).show(gui_ctx, |ui| {
            for (key, point) in self.points.iter() {
                draw_point(ui, point, key);
            }
        });
    }
}

fn generate_key_to_points() -> HashMap<String, Point> {
    let points = generate_points();
    let keys = generate_keys(points.len() as i32);
    keys.into_iter().zip(points).collect()
}

fn generate_points() -> Vec<Point> {
    let mut result = Vec::new();

    let step = 30.0;
    let of_set = 10.0;
    let (w, h) = rdev::display_size().unwrap();
    let h_dots = w / step as u64;
    let v_dots = h / step as u64;

    for v_index in 1..=v_dots {
        for h_index in 1..=h_dots {
            let x = h_index as f32 * step - of_set;
            let y = v_index as f32 * step - of_set;
            result.push(Point { x, y });
        }
    }
    result
}

fn generate_keys(size: i32) -> Vec<String> {
    fn generate_key_set(size: i32, source_set: Vec<String>) -> Vec<String> {
        let mut i = size;
        let mut new_generation = Vec::new();
        for source_key in &source_set {
            for letter in POINT_KEY_LETTERS {
                let new_key = format!("{source_key}{letter}");
                new_generation.push(new_key);
                i -= 1;
                if i == 0 {
                    return new_generation;
                }
            }
        }
        if i > 0 {
            let post_generation = generate_key_set(i, new_generation.clone());
            new_generation.extend(post_generation);
        }
        new_generation
    }

    let mut source_set = POINT_KEY_LETTERS
        .map(|letter| letter.to_string())
        .to_vec();
    let generated = generate_key_set(size - source_set.len() as i32, source_set.clone());
    source_set.extend(generated);
    source_set
}

fn draw_point(ui: &mut Ui, point: &Point, key: &str) {
    let position = Pos2::new(point.x, point.y);
    ui.painter().add(egui::epaint::CircleShape::filled(position, 0.6, Color32::RED));
    let painter = ui.painter();
    let galley = painter.layout(
        key.to_string(),
        FontId::new(10.0, FontFamily::Proportional),
        Color32::YELLOW,
        f32::INFINITY,
    );
    painter.add(egui::epaint::TextShape::new(position, galley));
}
