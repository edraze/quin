use std::collections::HashSet;
use egui_backend::egui;
use egui_backend::egui::{Color32, Context, FontId, Rect, Shape, Stroke, Ui};
use egui_backend::egui::epaint::RectShape;
use rdev::EventType;
use crate::common;
use crate::common::input_interceptor;
use crate::common::input_interceptor::Filter;
use crate::core::{Bind, Binding, Draw, Handler, Identify, Label, State};
// use crate::registry::mb_emulation_handler;
// use crate::registry::precise_mode_handler;

const GRID_DENSITY: u32 = 15;

const GM_ACTIVATE: &str = "gm_activate";
const GM_CHOOSE_TOP_LEFT: &str = "gm_choose_top_left";
const GM_CHOOSE_TOP_RIGHT: &str = "gm_choose_top_right";
const GM_CHOOSE_BOTTOM_LEFT: &str = "gm_choose_bottom_left";
const GM_CHOOSE_BOTTOM_RIGHT: &str = "gm_choose_bottom_right";
const LABEL_LETTERS: [&str; 21] = ["a", "b", "c", "d", "e", "f", "g", /*"h",*/ "i", /*"j", "k", "l",*/ "m", "n", "o", "p", "q", "r", "s", "t", "u", /*"v",*/ "w", "x", "y", "z"]; // todo replace by range

// todo refactoring
pub struct NeoGridModeHandler {
    is_mode_active: bool,
    selected_region: Option<Region>,
    regions: Vec<Region>,
    points: Vec<Point>,
}

impl Default for NeoGridModeHandler {
    fn default() -> Self {
        let regions = generate_regions();
        let points = generate_points_for_regions(&regions);
        NeoGridModeHandler { is_mode_active: false, selected_region: None, regions, points }
    }
}

#[derive(Clone)]
struct Region {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: String,
}

impl Region {
    fn get_end_x(&self) -> f32 {
        self.x + self.width
    }

    fn get_end_y(&self) -> f32 {
        self.y + self.height
    }

    fn get_width_middle(&self) -> f32 {
        self.x + self.width / 2.0
    }

    fn get_height_middle(&self) -> f32 {
        self.y + self.height / 2.0
    }

    fn get_middle(&self) -> (f32, f32) {
        (self.get_width_middle(), self.get_height_middle())
    }

    fn is_inside(&self, point: &Point) -> bool {
        point.x >= self.x && point.x <= self.get_end_x() &&
            point.y >= self.y && point.y <= self.get_end_y()
    }

    fn draw(&self, ui: &mut Ui) {
        let position = (self.x, self.y);
        let size = (self.width, self.height);
        let rect = Rect::from_min_size(position.into(), size.into());
        let stroke = Stroke::new(0.5, Color32::GREEN);
        let rect = Shape::Rect(RectShape {
            rect,
            rounding: Default::default(),
            fill: Color32::TRANSPARENT,
            stroke,
        });
        ui.painter().add(rect);

        draw_label(ui, &self.label, self.get_middle())
    }

    // todo remove duplication
    fn draw_ruler(&self, ui: &mut Ui) {
        let padding = GRID_DENSITY;

        let vertical_lines_count = self.width as u32 / padding;
        let mut label_iter = LABEL_LETTERS.into_iter();
        for v_l_index in 0..=vertical_lines_count {
            let x = self.x + (padding * v_l_index) as f32;
            let start = (x, self.y);
            let end = (x, self.get_end_y());
            draw_line(ui, start, end);

            let label = label_iter.next().expect("Not enough len of LABEL_LETTERS");
            draw_label(ui, label, (x, self.y));
        }

        let mut label_iter = LABEL_LETTERS.into_iter();
        let horizontal_lines_count = self.height as u32 / padding;
        for h_l_index in 0..=horizontal_lines_count {
            let y = self.y + (padding * h_l_index) as f32;
            let start = (self.x, y);
            let end = (self.get_end_x(), y);
            draw_line(ui, start, end);

            let label = label_iter.next().expect("Not enough len of LABEL_LETTERS");
            draw_label(ui, label, (self.x, y));
        }
    }
}

#[derive(Default, Debug)]
struct Point {
    x: f32,
    y: f32,
    label: String,
}

impl Point {
    fn draw(&self, ui: &mut Ui) {
        let position = (self.x, self.y);
        ui.painter().add(egui::epaint::CircleShape::filled(position.into(), 1.0, Color32::RED));
    }
}

impl Bind for NeoGridModeHandler {
    fn get_bindings(&self) -> Vec<Binding> {
        let mut bindings: Vec<Binding> = self.regions.iter()
            .map(|region| {
                let label = region.label.clone();
                let default_input = common::keys::string_to_event_buffer(&label);
                Binding { label, default_input }
            })
            .collect();

        let point_labels: HashSet<String> = self.points.iter()
            .map(|point| point.label.clone())
            .collect();

        for label in point_labels {
            let default_input = common::keys::string_to_event_buffer(&label);
            bindings.push(Binding { label, default_input });
        }

        bindings.push(Binding { label: GM_ACTIVATE.to_string(), default_input: "RAltRight".to_string() });
        bindings
    }
}

impl Draw for NeoGridModeHandler {
    fn draw(&self, gui_ctx: &Context) {
        if self.is_mode_active {
            self.draw(gui_ctx);
        } else {
            common::gui::init_frame(gui_ctx);
        }
    }
}

impl Identify for NeoGridModeHandler {
    fn get_id(&self) -> String {
        "neogrid".to_string()
    }
}

impl Handler for NeoGridModeHandler {
    fn execute(&mut self, label: &Label, _: &mut State) {
        if let Label::Keys(label) = label {
            if label.eq(GM_ACTIVATE) {
                self.toggle_mode();
            } else if self.is_mode_active {
                match label.as_str() {
                    // precise_mode_handler::PM_ACTIVATE |
                    // mb_emulation_handler::MB_ACTIVATE |
                    // mb_emulation_handler::MB_LEFT => self.toggle_mode(),
                    _ => {
                        if let Some(region) = &self.selected_region {
                            if let Some(point) = self.points.iter()
                                .find(|point| region.is_inside(point) && point.label.eq(label)) {
                                self.selected_region = None;
                                move_mouse(point.x, point.y);
                            }
                        } else if let Some(region) = self.regions.iter()
                            .find(|region| region.label.eq(label)) {
                            self.selected_region = Some(region.clone());
                            let (x, y) = region.get_middle();
                            move_mouse(x, y);
                        }
                    }
                }
            }
        }
    }
}

impl NeoGridModeHandler {
    fn toggle_mode(&mut self) {
        if self.is_mode_active {
            input_interceptor::remove_filter(Filter::BlockAll);
            self.selected_region = None;
            self.is_mode_active = false;
        } else {
            input_interceptor::filter(Filter::BlockAll);
            self.is_mode_active = true;
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

        egui::CentralPanel::default().frame(panel_frame).show(gui_ctx, |ui| {
            if let Some(region) = &self.selected_region {
                region.draw_ruler(ui);
                self.points.iter()
                    .filter(|point| region.is_inside(point))
                    .for_each(|point| point.draw(ui));
            } else {
                self.regions.iter()
                    .for_each(|region| region.draw(ui));
            }
        });
    }
}

fn draw_line(ui: &mut Ui, start: (f32, f32), end: (f32, f32)) {
    let stroke = Stroke::new(0.5, Color32::GREEN);
    let line = Shape::line(vec![
        start.into(),
        end.into(),
    ], stroke);
    ui.painter().add(line);
}

fn draw_label(ui: &mut Ui, label: &str, position: (f32, f32)) {
    let galley = ui.painter().layout(
        label.to_string(),
        FontId::proportional(10.0),
        Color32::YELLOW,
        f32::INFINITY,
    );
    let text_shape = egui::epaint::TextShape::new(position.into(), galley);

    let text_background = Shape::Rect(RectShape {
        rect: text_shape.visual_bounding_rect().expand(1.0),
        rounding: 3.0.into(),
        fill: Color32::from_rgb(30, 31, 34),
        stroke: Stroke::none(),
    });
    ui.painter().add(text_background);
    ui.painter().add(text_shape);
}

fn generate_regions() -> Vec<Region> {
    let elements_count = LABEL_LETTERS.len() * LABEL_LETTERS.len();
    let mut labels_iter = generate_labels(elements_count as u64).into_iter();

    let (display_width, display_height) = rdev::display_size().unwrap();
    let region_width = display_width as f32 / LABEL_LETTERS.len() as f32;
    let region_height = display_height as f32 / LABEL_LETTERS.len() as f32;

    let mut result = Vec::new();
    for v_index in 0..LABEL_LETTERS.len() {
        for h_index in 0..LABEL_LETTERS.len() {
            let label = labels_iter.next().expect("Not enough labels generated");
            let x = region_width * h_index as f32;
            let y = region_height * v_index as f32;
            let region = Region { x, y, width: region_width, height: region_height, label };
            result.push(region);
        }
    }
    result
}

fn generate_points_for_regions(regions: &Vec<Region>) -> Vec<Point> {
    regions.iter()
        .flat_map(generate_points_for_region)
        .collect()
}

fn generate_points_for_region(region: &Region) -> Vec<Point> {
    let padding = GRID_DENSITY;
    let h_points_count = region.width as u32 / padding;
    let v_points_count = region.height as u32 / padding;

    let mut v_label_iter = LABEL_LETTERS.into_iter();
    let mut result = Vec::new();
    for v_index in 0..=v_points_count {
        let v_label = v_label_iter.next().expect("Not enough len of LABEL_LETTERS");
        let mut h_label_iter = LABEL_LETTERS.into_iter();
        for h_index in 0..=h_points_count {
            let x = region.x + (h_index * padding) as f32;
            let y = region.y + (v_index * padding) as f32;

            let h_label = h_label_iter.next().expect("Not enough len of LABEL_LETTERS");
            let label = format!("{v_label}{h_label}");
            let point = Point { x, y, label };
            result.push(point);
        }
    }
    result
}

fn generate_labels(size: u64) -> Vec<String> {
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

fn move_mouse(x: f32, y: f32) {
    rdev::simulate(&EventType::MouseMove { x: x as f64, y: y as f64 }).unwrap();
}
