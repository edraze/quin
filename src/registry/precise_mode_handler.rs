use std::collections::HashMap;
use std::default::Default;
use enigo::{Enigo, MouseControllable};
use serde::Deserialize;
use crate::common::input_interceptor;
use crate::common::input_interceptor::Filter;
use crate::core::{Bind, Binding, Draw, Handler, Identify, Label, State};
use crate::core::Event::{KeyPress, KeyRelease};
use crate::core::Key::{AltLeft, AltRight, KeyH, KeyJ, KeyK, KeyL};

pub const HANDLER_ID: &str = "precise-mode-handler";
const PM_TOGGLE: &str = "pm_toggle";
pub const PM_ACTIVATE: &str = "pm_activate";
const PM_DEACTIVATE: &str = "pm_deactivate";
const PM_MOVE_LEFT: &str = "pm_move_left";
const PM_MOVE_RIGHT: &str = "pm_move_right";
const PM_MOVE_TOP: &str = "pm_move_top";
const PM_MOVE_BOTTOM: &str = "pm_move_bottom";

#[derive(Deserialize)]
pub struct PreciseModeConfig {
    cursor_speed: i32,
    bindings: HashMap<String, String>,
}

impl Default for PreciseModeConfig {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(PM_TOGGLE.to_string(), KeyRelease(AltRight).to_string());
        bindings.insert(PM_ACTIVATE.to_string(), KeyPress(AltLeft).to_string());
        bindings.insert(PM_DEACTIVATE.to_string(), KeyRelease(AltLeft).to_string());
        bindings.insert(PM_MOVE_LEFT.to_string(), KeyPress(KeyH).to_string());
        bindings.insert(PM_MOVE_RIGHT.to_string(), KeyPress(KeyL).to_string());
        bindings.insert(PM_MOVE_TOP.to_string(), KeyPress(KeyK).to_string());
        bindings.insert(PM_MOVE_BOTTOM.to_string(), KeyPress(KeyJ).to_string());

        PreciseModeConfig { cursor_speed: 5, bindings }
    }
}

pub struct PreciseModeHandler {
    config: PreciseModeConfig,
    is_mode_active: bool,
    enigo: Enigo,
}

impl Bind for PreciseModeHandler {
    fn get_bindings(&self) -> Vec<Binding> {
        self.config.bindings.clone()
            .into_iter()
            .map(|(label, default_input)| Binding { label, default_input })
            .collect()
    }
}

impl Identify for PreciseModeHandler {
    fn get_id(&self) -> String {
        HANDLER_ID.to_string()
    }
}

impl Draw for PreciseModeHandler {}

impl Handler for PreciseModeHandler {
    fn execute(&mut self, label: &Label, _: &mut State) {
        if let Label::Keys(label) = label {
            match label.as_str() {
                PM_TOGGLE => self.toggle_mode(),
                PM_ACTIVATE => self.activate_mode(),
                PM_DEACTIVATE => self.deactivate_mode(),

                PM_MOVE_LEFT => if self.is_mode_active { self.move_cursor_left_relatively(self.config.cursor_speed) },
                PM_MOVE_RIGHT => if self.is_mode_active { self.move_cursor_right_relatively(self.config.cursor_speed) },
                PM_MOVE_TOP => if self.is_mode_active { self.move_cursor_top_relatively(self.config.cursor_speed) },
                PM_MOVE_BOTTOM => if self.is_mode_active { self.move_cursor_botttom_relatively(self.config.cursor_speed) },
                _ => {}
            }
        }
    }
}

impl PreciseModeHandler {
    pub fn new(config: PreciseModeConfig) -> Self {
        Self { is_mode_active: false, config, enigo: Enigo::default() }
    }

    fn activate_mode(&mut self) {
        self.is_mode_active = true;
    }

    fn deactivate_mode(&mut self) {
        self.is_mode_active = false;
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

    fn move_cursor_left_relatively(&mut self, speed: i32) {
        self.enigo.mouse_move_relative(-speed, 0);
    }

    fn move_cursor_right_relatively(&mut self, speed: i32) {
        self.enigo.mouse_move_relative(speed, 0);
    }

    fn move_cursor_top_relatively(&mut self, speed: i32) {
        self.enigo.mouse_move_relative(0, speed);
    }

    fn move_cursor_botttom_relatively(&mut self, speed: i32) {
        self.enigo.mouse_move_relative(0, -speed);
    }
}
