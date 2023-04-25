use std::collections::HashMap;
use rdev::Button::{Left, Middle, Right};
use rdev::{Button, EventType};
use crate::common::input_interceptor;
use crate::common::input_interceptor::Filter;
use crate::core::{Bind, Binding, Draw, Handler, Identify, Label, State};
use serde::Deserialize;
use crate::core::Event::{KeyPress, KeyRelease};
use crate::core::Key::{AltLeft, AltRight, Comma, Dot, KeyV, Quote, SemiColon, Slash};

pub const HANDLER_ID: &str = "mouse-buttons-emulation-handler";
const MB_TOGGLE: &str = "mb_toggle";
pub const MB_ACTIVATE: &str = "mb_activate";
const MB_DEACTIVATE: &str = "mb_deactivate";
pub const MB_LEFT: &str = "mb_left";
const MB_RIGHT: &str = "mb_right";
const MB_MIDDLE: &str = "mb_middle";
const MB_SCROLL_UP: &str = "mb_scroll_up";
const MB_SCROLL_DOWN: &str = "mb_scroll_down";
const MB_DRAG_AND_DROP: &str = "mb_drag_and_drop";

#[derive(Deserialize)]
pub struct MButtonsEmulationConfig {
    scroll_speed: i64,
    bindings: HashMap<String, String>,
}

impl Default for MButtonsEmulationConfig {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(MB_TOGGLE.to_string(), KeyRelease(AltRight).to_string());
        bindings.insert(MB_ACTIVATE.to_string(), KeyPress(AltLeft).to_string());
        bindings.insert(MB_DEACTIVATE.to_string(), KeyRelease(AltLeft).to_string());
        bindings.insert(MB_LEFT.to_string(), KeyPress(SemiColon).to_string());
        bindings.insert(MB_RIGHT.to_string(), KeyPress(Quote).to_string());
        bindings.insert(MB_MIDDLE.to_string(), KeyPress(Slash).to_string());
        bindings.insert(MB_SCROLL_UP.to_string(), KeyPress(Dot).to_string());
        bindings.insert(MB_SCROLL_DOWN.to_string(), KeyPress(Comma).to_string());
        bindings.insert(MB_DRAG_AND_DROP.to_string(), KeyPress(KeyV).to_string());

        Self { scroll_speed: 2, bindings }
    }
}

pub struct MButtonsEmulationHandler {
    config: MButtonsEmulationConfig,
    is_mode_active: bool,
    is_drag_and_drop_active: bool,
}

impl Bind for MButtonsEmulationHandler {
    fn get_bindings(&self) -> Vec<Binding> {
        self.config.bindings.clone()
            .into_iter()
            .map(|(label, default_input)| Binding { label, default_input })
            .collect()
    }
}

impl Identify for MButtonsEmulationHandler {
    fn get_id(&self) -> String {
        HANDLER_ID.to_string()
    }
}

impl Draw for MButtonsEmulationHandler {}

impl Handler for MButtonsEmulationHandler {
    fn execute(&mut self, label: &Label, _: &mut State) {
        if let Label::Keys(label) = label {
            match label.as_str() {
                MB_TOGGLE => self.toggle_mode(),
                MB_ACTIVATE => self.activate_mode(),
                MB_DEACTIVATE => self.deactivate_mode(),

                MB_LEFT => if self.is_mode_active { emulate(Left) },
                MB_RIGHT => if self.is_mode_active { emulate(Right) },
                MB_MIDDLE => if self.is_mode_active { emulate(Middle) },
                MB_SCROLL_UP => if self.is_mode_active { scroll_up(self.config.scroll_speed) },
                MB_SCROLL_DOWN => if self.is_mode_active { scroll_down(self.config.scroll_speed) },
                MB_DRAG_AND_DROP => self.toggle_drag_and_drop(),
                _ => {}
            }
        }
    }
}

impl MButtonsEmulationHandler {
    pub fn new(config: MButtonsEmulationConfig) -> Self {
        Self { config, is_mode_active: false, is_drag_and_drop_active: false }
    }

    fn activate_mode(&mut self) {
        self.is_mode_active = true;
    }

    fn deactivate_mode(&mut self) {
        self.is_mode_active = false;
        self.toggle_drag_and_drop();
    }

    fn toggle_mode(&mut self) {
        if self.is_mode_active {
            input_interceptor::remove_filter(Filter::BlockAll);
            self.deactivate_mode();
        } else {
            input_interceptor::filter(Filter::BlockAll);
            self.activate_mode();
        }
    }

    fn toggle_drag_and_drop(&mut self) {
        if self.is_mode_active && !self.is_drag_and_drop_active {
            rdev::simulate(&EventType::ButtonPress(Left)).unwrap();
            self.is_drag_and_drop_active = true;
        } else if self.is_drag_and_drop_active {
            rdev::simulate(&EventType::ButtonRelease(Left)).unwrap();
            self.is_drag_and_drop_active = false;
        }
    }
}

fn emulate(button: Button) {
    rdev::simulate(&EventType::ButtonPress(button)).unwrap();
    rdev::simulate(&EventType::ButtonRelease(button)).unwrap();
}

fn scroll_up(speed: i64) {
    rdev::simulate(&EventType::Wheel { delta_x: 0, delta_y: speed * -1 }).unwrap();
}

fn scroll_down(speed: i64) {
    rdev::simulate(&EventType::Wheel { delta_x: 0, delta_y: speed }).unwrap();
}
