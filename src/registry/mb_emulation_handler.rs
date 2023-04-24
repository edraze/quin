use rdev::Button::{Left, Middle, Right};
use rdev::{Button, EventType};
use crate::common::input_interceptor;
use crate::common::input_interceptor::Filter;
use crate::core::{Bind, Binding, Draw, Handler, Label, State};

const MB_TOGGLE: &str = "mb_toggle";
pub const MB_ACTIVATE: &str = "mb_activate";
const MB_DEACTIVATE: &str = "mb_deactivate";
pub const MB_LEFT: &str = "mb_left";
const MB_RIGHT: &str = "mb_right";
const MB_MIDDLE: &str = "mb_middle";
const MB_SCROLL_UP: &str = "mb_scroll_up";
const MB_SCROLL_DOWN: &str = "mb_scroll_down";
const MB_DRAG_AND_DROP: &str = "mb_drag_and_drop";

#[derive(Default)]
pub struct MButtonsEmulationHandler {
    is_mode_active: bool,
    is_drag_and_drop_active: bool,
}

impl Bind for MButtonsEmulationHandler {
    fn get_bindings(&self) -> Vec<Binding> {
        vec![
            Binding { label: MB_TOGGLE.to_string(), default_input: "RAltRight".to_string() },
            Binding { label: MB_ACTIVATE.to_string(), default_input: "PAltLeft".to_string() },
            Binding { label: MB_DEACTIVATE.to_string(), default_input: "RAltLeft".to_string() },
            Binding { label: MB_LEFT.to_string(), default_input: "PSemiColon".to_string() },
            Binding { label: MB_RIGHT.to_string(), default_input: "PQuote".to_string() },
            Binding { label: MB_MIDDLE.to_string(), default_input: "PSlash".to_string() },
            Binding { label: MB_SCROLL_UP.to_string(), default_input: "PDot".to_string() },
            Binding { label: MB_SCROLL_DOWN.to_string(), default_input: "PComma".to_string() },
            Binding { label: MB_DRAG_AND_DROP.to_string(), default_input: "PKeyV".to_string() },
        ]
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
                MB_SCROLL_UP => if self.is_mode_active { scroll_up() },
                MB_SCROLL_DOWN => if self.is_mode_active { scroll_down() },
                MB_DRAG_AND_DROP => self.toggle_drag_and_drop(),
                _ => {}
            }
        }
    }
}

impl MButtonsEmulationHandler {
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

fn scroll_up() {
    rdev::simulate(&EventType::Wheel { delta_x: 0, delta_y: -2 }).unwrap();
}

fn scroll_down() {
    rdev::simulate(&EventType::Wheel { delta_x: 0, delta_y: 2 }).unwrap();
}
