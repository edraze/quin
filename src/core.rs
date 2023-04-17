use std::fmt;
use std::sync::{Arc, Mutex};
use egui_backend::egui;

pub enum Label {
    Keys(String)
}

pub struct Binding {
    pub label: String,
    pub default_input: String,
}

pub trait Bind {
    fn get_bindings(&self) -> Vec<Binding>;
}

pub trait Draw {
    fn draw(&self, _: &egui::Context) {}
}

pub trait Handler: Bind + Draw {
    fn execute(&mut self, label: &Label, state: &mut State);
}

pub struct State {}

pub struct Executor {
    handlers: Arc<Vec<Mutex<Box<dyn Handler + Send>>>>,
    state: State,
}

impl Executor {
    pub fn new(handlers: Arc<Vec<Mutex<Box<dyn Handler + Send>>>>, state: State) -> Executor {
        Executor { handlers, state }
    }

    pub fn run(&mut self, label: &Label) {
        self.handlers.iter()
            .for_each(|handler| handler.lock()
                .expect("Handler already locked")
                .execute(label, &mut self.state))
    }

    pub fn draw(&mut self, gui_ctx: &egui::Context) {
        self.handlers.iter()
            .for_each(|handler| handler.lock()
                .expect("Handler already locked")
                .draw(gui_ctx))
    }
}

#[derive(Debug)]
pub enum Key {
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    LeftArrow,
    MetaLeft,
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
    Function,
    Unknown,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
