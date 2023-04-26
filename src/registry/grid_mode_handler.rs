use std::collections::HashMap;
use egui_backend::egui;
use egui_backend::egui::{Color32, Context, FontId, Pos2, Shape, Stroke, Ui};
use egui_backend::egui::epaint::RectShape;
use rdev::EventType;
use crate::common;
use crate::common::input_interceptor;
use crate::common::input_interceptor::Filter;
use crate::core::{Bind, Binding, Draw, Handler, Identify, Label, State};
use crate::registry::mb_emulation_handler;
use crate::registry::precise_mode_handler;
use serde::Deserialize;
use crate::core::Event::KeyRelease;
use crate::core::Key::AltRight;

pub const HANDLER_ID: &str = "grid-mode-handler";
const GM_ACTIVATE: &str = "gm_activate";
const LABEL_LETTERS: [&str; 21] = ["a", "b", "c", "d", "e", "f", "g", /*"h",*/ "i", /*"j", "k", "l",*/ "m", "n", "o", "p", "q", "r", "s", "t", "u", /*"v",*/ "w", "x", "y", "z"]; // todo replace by range

#[derive(Deserialize)]
pub struct GridModeConfig {
    #[serde(default = "GridModeConfig::default_pivot_density")]
    pivot_grid_density_px: f32,
    #[serde(default = "GridModeConfig::default_bindings")]
    bindings: HashMap<String, String>,
}

impl GridModeConfig {
    fn default_pivot_density() -> f32 {
        20.0
    }

    fn default_bindings() -> HashMap<String, String> {
        let mut bindings = HashMap::new();
        bindings.insert(GM_ACTIVATE.to_string(), KeyRelease(AltRight).to_string());
        bindings
    }
}

impl Default for GridModeConfig {
    fn default() -> Self {
        Self { pivot_grid_density_px: Self::default_pivot_density(), bindings: Self::default_bindings() }
    }
}

pub struct GridModeHandler {
    config: GridModeConfig,
    is_mode_active: bool,
    points: Vec<Point>,
    is_pivot_active: bool,
    pivot: Pivot,
}

#[derive(Debug)]
struct Pivot {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    density: f32,
    points: Vec<Point>,
}

impl Pivot {
    fn new(density: f32) -> Self {
        let (x, y) = (0.0, 0.0);
        let letters_count = LABEL_LETTERS.len() as f32 - 1.0;
        let (display_width, display_height) = rdev::display_size().unwrap();
        let (pivot_width, pivot_height) = (display_width as f32 / letters_count, display_height as f32 / letters_count);
        let points = build_points_grid_for_rect(x, y, pivot_width, pivot_height, density, density);
        Pivot { x, y, width: pivot_width, height: pivot_height, density, points }
    }

    fn draw(&self, ui: &mut Ui) {
        for point in &self.points {
            point.draw(ui);
        }
    }

    fn update_points(&mut self) {
        self.points = build_points_grid_for_rect(self.x, self.y, self.width, self.height, self.density, self.density);
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x - self.width / 2.0;
        self.y = y - self.height / 2.0;
        self.update_points();
    }
}

#[derive(Default, Debug)]
struct Point {
    label: String,
    x: f32,
    y: f32,
}

impl Point {
    fn draw(&self, ui: &mut Ui) {
        let painter = ui.painter();
        let position = Pos2::new(self.x, self.y);

        painter.add(egui::epaint::CircleShape::filled(position, 1.0, Color32::RED));

        let galley = painter.layout(
            self.label.clone(),
            FontId::proportional(10.0),
            Color32::YELLOW,
            f32::INFINITY,
        );
        let text_shape = egui::epaint::TextShape::new(position, galley);

        let text_background = Shape::Rect(RectShape {
            rect: text_shape.visual_bounding_rect().expand(1.0),
            rounding: 3.0.into(),
            fill: Color32::from_rgb(30, 31, 34),
            stroke: Stroke::none(),
        });
        painter.add(text_background);
        painter.add(text_shape);
    }
}

impl Bind for GridModeHandler {
    fn get_bindings(&self) -> Vec<Binding> {
        let mut bindings: Vec<Binding> = self.points.iter()
            .chain(self.pivot.points.iter())
            .map(|point| {
                let label = point.label.clone();
                let default_input = common::keys::string_to_event_buffer(&label);
                Binding { label, default_input }
            })
            .collect();

        let mut static_bindings = GridModeConfig::default_bindings();
        static_bindings.extend(self.config.bindings.clone());
        for (label, default_input) in static_bindings {
            bindings.push(Binding { label: label.clone(), default_input: default_input.clone() });
        }
        bindings
    }
}

impl Identify for GridModeHandler {
    fn get_id(&self) -> String {
        HANDLER_ID.to_string()
    }
}

impl Draw for GridModeHandler {
    fn draw(&self, gui_ctx: &Context) {
        if self.is_mode_active {
            self.draw(gui_ctx);
        }
    }
}

impl Handler for GridModeHandler {
    fn execute(&mut self, label: &Label, _: &mut State) {
        if let Label::Keys(label) = label {
            if label.eq(GM_ACTIVATE) {
                self.toggle_mode();
            } else if self.is_mode_active {
                match label.as_str() {
                    precise_mode_handler::PM_ACTIVATE |
                    mb_emulation_handler::MB_ACTIVATE |
                    mb_emulation_handler::MB_LEFT => self.toggle_mode(),
                    label => self.on_point_label(label)
                }
            }
        }
    }
}

impl GridModeHandler {
    pub fn new(config: GridModeConfig) -> Self {
        let points = build_points_grid();
        let pivot = Pivot::new(config.pivot_grid_density_px);
        Self { config, is_mode_active: false, points, is_pivot_active: false, pivot }
    }

    fn toggle_mode(&mut self) {
        if self.is_mode_active {
            input_interceptor::remove_filter(Filter::BlockAll);
            self.is_pivot_active = false;
            self.is_mode_active = false;
        } else {
            input_interceptor::filter(Filter::BlockAll);
            self.is_mode_active = true;
        }
    }

    fn on_point_label(&mut self, label: &str) {
        if self.is_pivot_active {
            if let Some(Point { label: _, x, y }) = self.pivot.points.iter()
                .find(|point| point.label.eq(label)) {
                rdev::simulate(&EventType::MouseMove { x: *x as f64, y: *y as f64 }).unwrap();
                self.is_pivot_active = false;
            }
        } else if let Some(Point { label: _, x, y }) = self.points.iter()
            .find(|point| point.label.eq(label)) {
            self.pivot.set_position(*x, *y);
            rdev::simulate(&EventType::MouseMove { x: *x as f64, y: *y as f64 }).unwrap();
            self.is_pivot_active = true;
        }
    }

    fn draw(&self, gui_ctx: &Context) {
        let panel_frame = egui::Frame {
            fill: Color32::TRANSPARENT,
            rounding: 0.0.into(),
            stroke: Stroke::none(),
            outer_margin: 0.0.into(),
            ..Default::default()
        };

        egui::CentralPanel::default().frame(panel_frame).show(gui_ctx, |ui|
            if self.is_pivot_active {
                self.pivot.draw(ui);
            } else {
                self.points.iter()
                    .for_each(|point| point.draw(ui));
            },
        );
    }
}

fn build_points_grid() -> Vec<Point> {
    let letters_count = LABEL_LETTERS.len() as f32 - 1.0;
    let (pivot_width, pivot_height) = rdev::display_size().unwrap();
    let (x_padding, y_padding) = (pivot_width as f32 / letters_count, pivot_height as f32 / letters_count);
    let (pivot_x, pivot_y) = (x_padding / 2.0, y_padding / 2.0);
    build_points_grid_for_rect(pivot_x, pivot_y, pivot_width as f32, pivot_height as f32, x_padding, y_padding)
}

fn build_points_grid_for_rect(x_start: f32, y_start: f32, width: f32, height: f32, x_padding: f32, y_padding: f32) -> Vec<Point> {
    let mut result = Vec::new();
    let x_points_count = width / x_padding;
    let y_points_count = height / y_padding;

    let mut y_label_iter = LABEL_LETTERS.into_iter();
    for y_index in 0..=y_points_count as i32 {
        let y_label = y_label_iter.next().expect("Not enough len of LABEL_LETTERS");
        let mut x_label_iter = LABEL_LETTERS.into_iter();
        for x_index in 0..=x_points_count as i32 {
            let x = x_start + x_index as f32 * x_padding;
            let y = y_start + y_index as f32 * y_padding;

            let x_label = x_label_iter.next().expect("Not enough len of LABEL_LETTERS");
            let label = format!("{y_label}{x_label}");

            let point = Point { x, y, label };
            result.push(point);
        }
    }
    result
}

fn generate_keys(size: u64) -> Vec<String> {
    fn generate_key_set(size: u64, source_set: Vec<String>) -> Vec<String> {
        let mut i = size;
        let mut new_generation = Vec::new();
        for source_key in &source_set {
            for letter in LABEL_LETTERS {
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

    let source_set = LABEL_LETTERS
        .map(|letter| letter.to_string())
        .to_vec();
    generate_key_set(size, source_set)
}
