use enigo::{Enigo, MouseControllable};
use crate::core::{Bind, Binding, Draw, Handler, Label, State};

const PM_ACTIVATE: &str = "pm_activate";
const PM_DEACTIVATE: &str = "pm_deactivate";
const PM_MOVE_LEFT: &str = "pm_move_left";
const PM_MOVE_RIGHT: &str = "pm_move_right";
const PM_MOVE_TOP: &str = "pm_move_top";
const PM_MOVE_BOTTOM: &str = "pm_move_bottom";

const CURSOR_SPEED: i32 = 5;

#[derive(Default)]
pub struct PreciseModeHandler {
    is_mode_active: bool,
    enigo: Enigo,
}

impl Bind for PreciseModeHandler {
    fn get_bindings(&self) -> Vec<Binding> {
        vec![
            Binding { label: PM_ACTIVATE.to_string(), default_input: "PAltRight".to_string() },
            Binding { label: PM_DEACTIVATE.to_string(), default_input: "RAltRight".to_string() },
            Binding { label: PM_MOVE_LEFT.to_string(), default_input: "PKeyH".to_string() },
            Binding { label: PM_MOVE_RIGHT.to_string(), default_input: "PKeyL".to_string() },
            Binding { label: PM_MOVE_TOP.to_string(), default_input: "PKeyK".to_string() },
            Binding { label: PM_MOVE_BOTTOM.to_string(), default_input: "PKeyJ".to_string() },
        ]
    }
}

impl Draw for PreciseModeHandler {}

impl Handler for PreciseModeHandler {
    fn execute(&mut self, label: &Label, _: &mut State) {
        if let Label::Keys(label) = label {
            match label.as_str() {
                PM_ACTIVATE => self.activate_mode(),
                PM_DEACTIVATE => self.deactivate_mode(),

                PM_MOVE_LEFT => if self.is_mode_active { self.move_cursor_left_relatively(CURSOR_SPEED) },
                PM_MOVE_RIGHT => if self.is_mode_active { self.move_cursor_right_relatively(CURSOR_SPEED) },
                PM_MOVE_TOP => if self.is_mode_active { self.move_cursor_top_relatively(CURSOR_SPEED) },
                PM_MOVE_BOTTOM => if self.is_mode_active { self.move_cursor_botttom_relatively(CURSOR_SPEED) },
                _ => return
            }

        }
    }
}

impl PreciseModeHandler {
    fn activate_mode(&mut self) {
        self.is_mode_active = true
    }

    fn deactivate_mode(&mut self) {
        self.is_mode_active = false
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
