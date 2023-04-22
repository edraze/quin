use egui_backend::egui;
use egui_backend::egui::{Color32, Context, FontFamily, FontId, Pos2, Rect, Shape, Stroke, TextStyle, Ui};
use egui_backend::egui::epaint::RectShape;
use rdev::EventType;
use crate::common;
use crate::common::input_interceptor;
use crate::common::input_interceptor::Filter;
use crate::core::{Bind, Binding, Draw, Handler, Label, State};
use crate::registry::mb_emulation_handler;
use crate::registry::precise_mode_handler;

const GM_ACTIVATE: &str = "gm_activate";
const GM_CHOOSE_TOP_LEFT: &str = "gm_choose_top_left";
const GM_CHOOSE_TOP_RIGHT: &str = "gm_choose_top_right";
const GM_CHOOSE_BOTTOM_LEFT: &str = "gm_choose_bottom_left";
const GM_CHOOSE_BOTTOM_RIGHT: &str = "gm_choose_bottom_right";
const POINT_KEY_LETTERS: [&str; 17] = ["a", "b", "c", /*"d", "e", "f",*/ "g", /*"h",*/ "i", /*"j", "k", "l",*/ "m", "n", "o", "p", "q", /*"r",*/ "s", "t", "u", /*"v",*/ "w", "x", "y", "z"]; // todo replace by range

pub struct GridModeHandler {
    is_mode_active: bool,
    points: Vec<Point>,
    region: Region,
}

impl Default for GridModeHandler {
    fn default() -> Self {
        let points = generate_points();
        GridModeHandler { is_mode_active: false, points, region: Region::default() }
    }
}

#[derive(Debug)]
struct Region {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Default for Region {
    fn default() -> Self {
        let (width, height) = rdev::display_size().unwrap();
        Region { x: 0.0, y: 0.0, width: width as f32, height: height as f32 }
    }
}

impl Region {
    fn get_top_left(&self) -> Self {
        Self { x: self.x, y: self.y, width: self.width / 2.0, height: self.height / 2.0 }
    }

    fn get_top_right(&self) -> Self {
        Self { x: self.get_width_middle(), y: self.y, width: self.width / 2.0, height: self.height / 2.0 }
    }

    fn get_bottom_left(&self) -> Self {
        Self { x: self.x, y: self.get_height_middle(), width: self.width / 2.0, height: self.height / 2.0 }
    }

    fn get_bottom_right(&self) -> Self {
        let (x, y) = self.get_middle();
        Self { x, y, width: self.width / 2.0, height: self.height / 2.0 }
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

    fn get_end_x(&self) -> f32 {
        self.x + self.width
    }

    fn get_end_y(&self) -> f32 {
        self.y + self.height
    }

    fn is_inside(&self, point: &Point) -> bool {
        point.x > self.x && point.x < self.get_end_x() &&
            point.y > self.y && point.y < self.get_end_y()
    }

    fn draw(&self, ui: &mut Ui) {
        let position = Pos2 { x: self.x, y: self.y };
        let size = (self.width, self.height);
        let rect = Rect::from_min_size(position, size.into());
        let stroke = Stroke::new(0.5, Color32::GREEN);
        let rect = Shape::Rect(RectShape {
            rect,
            rounding: Default::default(),
            fill: Color32::TRANSPARENT,
            stroke,
        });
        ui.painter().add(rect);

        let line = Shape::line(vec![
            Pos2::new(self.get_width_middle(), self.y),
            Pos2::new(self.get_width_middle(), self.get_end_y()),
        ], stroke);
        ui.painter().add(line);

        let line = Shape::line(vec![
            Pos2::new(self.x, self.get_height_middle()),
            Pos2::new(self.get_end_x(), self.get_height_middle()),
        ], stroke);
        ui.painter().add(line);
    }
}

#[derive(Default, Debug)]
struct Point {
    key: String,
    x: f32,
    y: f32,
}

impl Point {
    fn draw(&self, ui: &mut Ui) {
        let painter = ui.painter();
        let position = Pos2::new(self.x, self.y);

        painter.add(egui::epaint::CircleShape::filled(position, 1.0, Color32::RED));

        let galley = painter.layout(
            self.key.clone(),
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
            .map(|point| {
                let label = point.key.clone();
                let default_input = common::keys::string_to_event_buffer(&label);
                Binding { label, default_input }
            })
            .collect();
        bindings.push(Binding { label: GM_ACTIVATE.to_string(), default_input: "RAltRight".to_string() });
        bindings.push(Binding { label: GM_CHOOSE_TOP_LEFT.to_string(), default_input: "RKeyE".to_string() });
        bindings.push(Binding { label: GM_CHOOSE_TOP_RIGHT.to_string(), default_input: "RKeyR".to_string() });
        bindings.push(Binding { label: GM_CHOOSE_BOTTOM_LEFT.to_string(), default_input: "RKeyD".to_string() });
        bindings.push(Binding { label: GM_CHOOSE_BOTTOM_RIGHT.to_string(), default_input: "RKeyF".to_string() });
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
                self.toggle_mode();
            } else if self.is_mode_active {
                match label.as_str() {
                    precise_mode_handler::PM_ACTIVATE |
                    mb_emulation_handler::MB_ACTIVATE |
                    mb_emulation_handler::MB_LEFT => self.toggle_mode(),
                    GM_CHOOSE_TOP_LEFT => {
                        self.region = self.region.get_top_left();
                        self.move_mouse_to_region();
                    }
                    GM_CHOOSE_TOP_RIGHT => {
                        self.region = self.region.get_top_right();
                        self.move_mouse_to_region();
                    }
                    GM_CHOOSE_BOTTOM_LEFT => {
                        self.region = self.region.get_bottom_left();
                        self.move_mouse_to_region();
                    }
                    GM_CHOOSE_BOTTOM_RIGHT => {
                        self.region = self.region.get_bottom_right();
                        self.move_mouse_to_region();
                    }
                    _ => {
                        if let Some(Point { key: _, x, y }) = self.points.iter()
                            .find(|point| point.key.eq(label)) {
                            rdev::simulate(&EventType::MouseMove { x: *x as f64, y: *y as f64 }).unwrap();
                        }
                    }
                }
            }
        }
    }
}

impl GridModeHandler {
    fn toggle_mode(&mut self) {
        if self.is_mode_active {
            input_interceptor::remove_filter(Filter::BlockAll);
            self.region = Default::default();
            self.is_mode_active = false;
        } else {
            input_interceptor::filter(Filter::BlockAll);
            self.is_mode_active = true;
        }
    }

    fn draw_grid(&self, gui_ctx: &Context) {
        let panel_frame = egui::Frame {
            fill: Color32::TRANSPARENT,
            rounding: 0.0.into(),
            stroke: Stroke::none(),
            outer_margin: 0.0.into(),
            ..Default::default()
        };

        egui::CentralPanel::default().frame(panel_frame).show(gui_ctx, |ui| {
            self.region.draw(ui);
            self.points.iter()
                .filter(|point| self.region.is_inside(point))
                .for_each(|point| point.draw(ui));
        });
    }

    fn move_mouse_to_region(&self) {
        let (x, y) = self.region.get_middle();
        rdev::simulate(&EventType::MouseMove { x: x as f64, y: y as f64 }).unwrap();
    }
}

fn generate_points() -> Vec<Point> {
    let mut result = Vec::new();

    let step = 30.0;
    let of_set = 15.0;
    let (w, h) = rdev::display_size().unwrap();
    let v_dots = h / step as u64;
    let h_dots = w / step as u64;

    let mut keys_iter = generate_keys(v_dots * h_dots).into_iter();
    for v_index in 1..=v_dots {
        for h_index in 1..=h_dots {
            let key = keys_iter.next().expect("Not enough keys");
            let x = h_index as f32 * step - of_set;
            let y = v_index as f32 * step - of_set;
            result.push(Point { key, x, y });
        }
    }
    result
}

fn generate_keys(size: u64) -> Vec<String> {
    fn generate_key_set(size: u64, source_set: Vec<String>) -> Vec<String> {
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
    let generated = generate_key_set(size - source_set.len() as u64, source_set.clone());
    source_set.extend(generated);
    source_set
}
