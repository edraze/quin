use bevy::prelude::Event;
use rdev::EventType;
use crate::plugins::global_input::{all_keyboard_events, all_mouse_events};

#[derive(Event, Debug)]
pub enum InputFilterEvent {
    Block(FilterInput),
    Unblock(FilterInput),
}

#[derive(Debug)]
pub enum FilterInput {
    All,
    AllKeyboard,
    AllMouse,
    Keyboard(FilterKeyEvent),
    MouseButton(FilterButtonEvent),
    MouseMove,
}

impl FilterInput {
    pub fn to_input_events(&self) -> Vec<InputEvent> {
        match self {
            FilterInput::All => {
                let mut events = all_keyboard_events();
                events.append(&mut all_mouse_events());
                events
            }
            FilterInput::AllKeyboard => all_keyboard_events(),
            FilterInput::AllMouse => all_mouse_events(),
            FilterInput::Keyboard(filter_key) => {
                match filter_key {
                    FilterKeyEvent::Any(key) => vec![
                        InputEvent::Keyboard(KeyEvent::Pressed(*key)),
                        InputEvent::Keyboard(KeyEvent::Released(*key)),
                    ],
                    FilterKeyEvent::Pressed(key) => vec![InputEvent::Keyboard(KeyEvent::Pressed(*key))],
                    FilterKeyEvent::Released(key) => vec![InputEvent::Keyboard(KeyEvent::Released(*key))],
                }
            }
            FilterInput::MouseButton(filter_button) => {
                match filter_button {
                    FilterButtonEvent::Any(button) =>
                        vec![
                            InputEvent::MouseButton(ButtonEvent::Pressed(*button)),
                            InputEvent::MouseButton(ButtonEvent::Released(*button)),
                        ],
                    FilterButtonEvent::Pressed(button) => vec![InputEvent::MouseButton(ButtonEvent::Pressed(*button))],
                    FilterButtonEvent::Released(button) => vec![InputEvent::MouseButton(ButtonEvent::Released(*button))],
                }
            }
            FilterInput::MouseMove => vec![InputEvent::MouseMove { x: 0., y: 0. }]
        }
    }
}

#[derive(Event, Debug, PartialEq, Clone)]
pub enum InputEvent {
    Keyboard(KeyEvent),
    MouseButton(ButtonEvent),
    MouseMove {
        x: f64,
        y: f64,
    },
    Wheel {
        delta_x: i64,
        delta_y: i64,
    },
}

impl From<&rdev::Event> for InputEvent {
    fn from(value: &rdev::Event) -> Self {
        let event = value.event_type;
        match &event {
            EventType::KeyPress(key) => InputEvent::Keyboard(KeyEvent::Pressed(key.into())),
            EventType::KeyRelease(key) => InputEvent::Keyboard(KeyEvent::Released(key.into())),
            EventType::ButtonPress(button) => InputEvent::MouseButton(ButtonEvent::Pressed(button.into())),
            EventType::ButtonRelease(button) => InputEvent::MouseButton(ButtonEvent::Released(button.into())),
            EventType::MouseMove { x, y } => InputEvent::MouseMove { x: *x, y: *y },
            EventType::Wheel { delta_x, delta_y } => InputEvent::Wheel { delta_x: *delta_x, delta_y: *delta_y },
        }
    }
}

#[derive(Debug)]
pub enum FilterKeyEvent {
    Any(Key),
    Pressed(Key),
    Released(Key),
}

#[derive(Debug)]
pub enum FilterButtonEvent {
    Any(Button),
    Pressed(Button),
    Released(Button),
}


#[derive(Debug, PartialEq, Clone)]
pub enum ButtonEvent {
    Pressed(Button),
    Released(Button),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Button {
    Left,
    Right,
    Middle,
    Unknown(u8),
}

impl From<&rdev::Button> for Button {
    fn from(value: &rdev::Button) -> Self {
        match value {
            rdev::Button::Left => Button::Left,
            rdev::Button::Right => Button::Right,
            rdev::Button::Middle => Button::Middle,
            rdev::Button::Unknown(button) => Button::Unknown(*button),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Key {
    Alt,
    AltGr,
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
    Unknown(u32),
}

impl From<&rdev::Key> for Key {
    fn from(value: &rdev::Key) -> Self {
        match value {
            rdev::Key::Alt => Key::Alt,
            rdev::Key::AltGr => Key::AltGr,
            rdev::Key::Backspace => Key::Backspace,
            rdev::Key::CapsLock => Key::CapsLock,
            rdev::Key::ControlLeft => Key::ControlLeft,
            rdev::Key::ControlRight => Key::ControlRight,
            rdev::Key::Delete => Key::Delete,
            rdev::Key::DownArrow => Key::DownArrow,
            rdev::Key::End => Key::End,
            rdev::Key::Escape => Key::Escape,
            rdev::Key::F1 => Key::F1,
            rdev::Key::F10 => Key::F10,
            rdev::Key::F11 => Key::F11,
            rdev::Key::F12 => Key::F12,
            rdev::Key::F2 => Key::F2,
            rdev::Key::F3 => Key::F3,
            rdev::Key::F4 => Key::F4,
            rdev::Key::F5 => Key::F5,
            rdev::Key::F6 => Key::F6,
            rdev::Key::F7 => Key::F7,
            rdev::Key::F8 => Key::F8,
            rdev::Key::F9 => Key::F9,
            rdev::Key::Home => Key::Home,
            rdev::Key::LeftArrow => Key::LeftArrow,
            rdev::Key::MetaLeft => Key::MetaLeft,
            rdev::Key::MetaRight => Key::MetaRight,
            rdev::Key::PageDown => Key::PageDown,
            rdev::Key::PageUp => Key::PageUp,
            rdev::Key::Return => Key::Return,
            rdev::Key::RightArrow => Key::RightArrow,
            rdev::Key::ShiftLeft => Key::ShiftLeft,
            rdev::Key::ShiftRight => Key::ShiftRight,
            rdev::Key::Space => Key::Space,
            rdev::Key::Tab => Key::Tab,
            rdev::Key::UpArrow => Key::UpArrow,
            rdev::Key::PrintScreen => Key::PrintScreen,
            rdev::Key::ScrollLock => Key::ScrollLock,
            rdev::Key::Pause => Key::Pause,
            rdev::Key::NumLock => Key::NumLock,
            rdev::Key::BackQuote => Key::BackQuote,
            rdev::Key::Num1 => Key::Num1,
            rdev::Key::Num2 => Key::Num2,
            rdev::Key::Num3 => Key::Num3,
            rdev::Key::Num4 => Key::Num4,
            rdev::Key::Num5 => Key::Num5,
            rdev::Key::Num6 => Key::Num6,
            rdev::Key::Num7 => Key::Num7,
            rdev::Key::Num8 => Key::Num8,
            rdev::Key::Num9 => Key::Num9,
            rdev::Key::Num0 => Key::Num0,
            rdev::Key::Minus => Key::Minus,
            rdev::Key::Equal => Key::Equal,
            rdev::Key::KeyQ => Key::KeyQ,
            rdev::Key::KeyW => Key::KeyW,
            rdev::Key::KeyE => Key::KeyE,
            rdev::Key::KeyR => Key::KeyR,
            rdev::Key::KeyT => Key::KeyT,
            rdev::Key::KeyY => Key::KeyY,
            rdev::Key::KeyU => Key::KeyU,
            rdev::Key::KeyI => Key::KeyI,
            rdev::Key::KeyO => Key::KeyO,
            rdev::Key::KeyP => Key::KeyP,
            rdev::Key::LeftBracket => Key::LeftBracket,
            rdev::Key::RightBracket => Key::RightBracket,
            rdev::Key::KeyA => Key::KeyA,
            rdev::Key::KeyS => Key::KeyS,
            rdev::Key::KeyD => Key::KeyD,
            rdev::Key::KeyF => Key::KeyF,
            rdev::Key::KeyG => Key::KeyG,
            rdev::Key::KeyH => Key::KeyH,
            rdev::Key::KeyJ => Key::KeyJ,
            rdev::Key::KeyK => Key::KeyK,
            rdev::Key::KeyL => Key::KeyL,
            rdev::Key::SemiColon => Key::SemiColon,
            rdev::Key::Quote => Key::Quote,
            rdev::Key::BackSlash => Key::BackSlash,
            rdev::Key::IntlBackslash => Key::IntlBackslash,
            rdev::Key::KeyZ => Key::KeyZ,
            rdev::Key::KeyX => Key::KeyX,
            rdev::Key::KeyC => Key::KeyC,
            rdev::Key::KeyV => Key::KeyV,
            rdev::Key::KeyB => Key::KeyB,
            rdev::Key::KeyN => Key::KeyN,
            rdev::Key::KeyM => Key::KeyM,
            rdev::Key::Comma => Key::Comma,
            rdev::Key::Dot => Key::Dot,
            rdev::Key::Slash => Key::Slash,
            rdev::Key::Insert => Key::Insert,
            rdev::Key::KpReturn => Key::KpReturn,
            rdev::Key::KpMinus => Key::KpMinus,
            rdev::Key::KpPlus => Key::KpPlus,
            rdev::Key::KpMultiply => Key::KpMultiply,
            rdev::Key::KpDivide => Key::KpDivide,
            rdev::Key::Kp0 => Key::Kp0,
            rdev::Key::Kp1 => Key::Kp1,
            rdev::Key::Kp2 => Key::Kp2,
            rdev::Key::Kp3 => Key::Kp3,
            rdev::Key::Kp4 => Key::Kp4,
            rdev::Key::Kp5 => Key::Kp5,
            rdev::Key::Kp6 => Key::Kp6,
            rdev::Key::Kp7 => Key::Kp7,
            rdev::Key::Kp8 => Key::Kp8,
            rdev::Key::Kp9 => Key::Kp9,
            rdev::Key::KpDelete => Key::KpDelete,
            rdev::Key::Function => Key::Function,
            rdev::Key::Unknown(key) => Key::Unknown(*key),
        }
    }
}