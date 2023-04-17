use std::collections::{HashMap, HashSet};
use rdev::{Event, Key};
use rdev::EventType::{KeyPress, KeyRelease};
use crate::config::ConfigLoader;
use crate::core::{Label};
use crate::registry;

pub struct Translator {
    bindings: Vec<(String, String)>,
    input_buffer: Vec<String>,
}

impl Default for Translator {
    fn default() -> Self {
        let config = ConfigLoader::load_default();
        let input_buffer = Vec::with_capacity(8); // todo determinate capacity by max value from config

        let mut bindings: HashSet<(String, String)> = registry::get_bindings().into_iter()
            .map(|binding| (binding.label, binding.default_input))
            .collect();
        bindings.extend(config.bindings.into_iter().collect::<Vec<_>>());

        let mut bindings = Vec::from_iter(bindings);
        bindings.sort_by_key(|(_, input_buffer)| input_buffer.len());
        Translator { bindings, input_buffer }
    }
}

impl Translator {
    pub fn translate(&mut self, event: Event) -> Vec<Label> {
        let mut labels = Vec::new();

        if let KeyPress(key) = event.event_type {
            self.save_input(format!("P{}", Into::<crate::core::Key>::into(key)))
        } else if let KeyRelease(key) = event.event_type {
            self.save_input(format!("R{}", Into::<crate::core::Key>::into(key)))
        }
        let input_buffer_string = self.input_buffer.join("");
        for (label, hotkeys) in &self.bindings {
            if input_buffer_string.ends_with(hotkeys) {
                labels.push(Label::Keys(label.clone()))
            }
        }

        labels
    }

    fn save_input(&mut self, input: String) {
        if self.input_buffer.capacity() != 0 {
            if self.input_buffer.len() == self.input_buffer.capacity() {
                self.input_buffer.remove(0);
            }
            self.input_buffer.push(input);
        }
    }
}

impl Into<crate::core::Key> for Key {
    fn into(self) -> crate::core::Key {
        match self {
            Key::Alt => crate::core::Key::AltLeft,
            Key::AltGr => crate::core::Key::AltRight,
            Key::Backspace => crate::core::Key::Backspace,
            Key::CapsLock => crate::core::Key::CapsLock,
            Key::ControlLeft => crate::core::Key::ControlLeft,
            Key::ControlRight => crate::core::Key::ControlRight,
            Key::Delete => crate::core::Key::Delete,
            Key::DownArrow => crate::core::Key::DownArrow,
            Key::End => crate::core::Key::End,
            Key::Escape => crate::core::Key::Escape,
            Key::F1 => crate::core::Key::F1,
            Key::F10 => crate::core::Key::F10,
            Key::F11 => crate::core::Key::F11,
            Key::F12 => crate::core::Key::F12,
            Key::F2 => crate::core::Key::F2,
            Key::F3 => crate::core::Key::F3,
            Key::F4 => crate::core::Key::F4,
            Key::F5 => crate::core::Key::F5,
            Key::F6 => crate::core::Key::F6,
            Key::F7 => crate::core::Key::F7,
            Key::F8 => crate::core::Key::F8,
            Key::F9 => crate::core::Key::F9,
            Key::Home => crate::core::Key::Home,
            Key::LeftArrow => crate::core::Key::LeftArrow,
            Key::MetaLeft => crate::core::Key::MetaLeft,
            Key::MetaRight => crate::core::Key::MetaRight,
            Key::PageDown => crate::core::Key::PageDown,
            Key::PageUp => crate::core::Key::PageUp,
            Key::Return => crate::core::Key::Return,
            Key::RightArrow => crate::core::Key::RightArrow,
            Key::ShiftLeft => crate::core::Key::ShiftLeft,
            Key::ShiftRight => crate::core::Key::ShiftRight,
            Key::Space => crate::core::Key::Space,
            Key::Tab => crate::core::Key::Tab,
            Key::UpArrow => crate::core::Key::UpArrow,
            Key::PrintScreen => crate::core::Key::PrintScreen,
            Key::ScrollLock => crate::core::Key::ScrollLock,
            Key::Pause => crate::core::Key::Pause,
            Key::NumLock => crate::core::Key::NumLock,
            Key::BackQuote => crate::core::Key::BackQuote,
            Key::Num1 => crate::core::Key::Num1,
            Key::Num2 => crate::core::Key::Num2,
            Key::Num3 => crate::core::Key::Num3,
            Key::Num4 => crate::core::Key::Num4,
            Key::Num5 => crate::core::Key::Num5,
            Key::Num6 => crate::core::Key::Num6,
            Key::Num7 => crate::core::Key::Num7,
            Key::Num8 => crate::core::Key::Num8,
            Key::Num9 => crate::core::Key::Num9,
            Key::Num0 => crate::core::Key::Num0,
            Key::Minus => crate::core::Key::Minus,
            Key::Equal => crate::core::Key::Equal,
            Key::KeyQ => crate::core::Key::KeyQ,
            Key::KeyW => crate::core::Key::KeyW,
            Key::KeyE => crate::core::Key::KeyE,
            Key::KeyR => crate::core::Key::KeyR,
            Key::KeyT => crate::core::Key::KeyT,
            Key::KeyY => crate::core::Key::KeyY,
            Key::KeyU => crate::core::Key::KeyU,
            Key::KeyI => crate::core::Key::KeyI,
            Key::KeyO => crate::core::Key::KeyO,
            Key::KeyP => crate::core::Key::KeyP,
            Key::LeftBracket => crate::core::Key::LeftBracket,
            Key::RightBracket => crate::core::Key::RightBracket,
            Key::KeyA => crate::core::Key::KeyA,
            Key::KeyS => crate::core::Key::KeyS,
            Key::KeyD => crate::core::Key::KeyD,
            Key::KeyF => crate::core::Key::KeyF,
            Key::KeyG => crate::core::Key::KeyG,
            Key::KeyH => crate::core::Key::KeyH,
            Key::KeyJ => crate::core::Key::KeyJ,
            Key::KeyK => crate::core::Key::KeyK,
            Key::KeyL => crate::core::Key::KeyL,
            Key::SemiColon => crate::core::Key::SemiColon,
            Key::Quote => crate::core::Key::Quote,
            Key::BackSlash => crate::core::Key::BackSlash,
            Key::IntlBackslash => crate::core::Key::IntlBackslash,
            Key::KeyZ => crate::core::Key::KeyZ,
            Key::KeyX => crate::core::Key::KeyX,
            Key::KeyC => crate::core::Key::KeyC,
            Key::KeyV => crate::core::Key::KeyV,
            Key::KeyB => crate::core::Key::KeyB,
            Key::KeyN => crate::core::Key::KeyN,
            Key::KeyM => crate::core::Key::KeyM,
            Key::Comma => crate::core::Key::Comma,
            Key::Dot => crate::core::Key::Dot,
            Key::Slash => crate::core::Key::Slash,
            Key::Insert => crate::core::Key::Insert,
            Key::KpReturn => crate::core::Key::KpReturn,
            Key::KpMinus => crate::core::Key::KpMinus,
            Key::KpPlus => crate::core::Key::KpPlus,
            Key::KpMultiply => crate::core::Key::KpMultiply,
            Key::KpDivide => crate::core::Key::KpDivide,
            Key::Kp0 => crate::core::Key::Kp0,
            Key::Kp1 => crate::core::Key::Kp1,
            Key::Kp2 => crate::core::Key::Kp2,
            Key::Kp3 => crate::core::Key::Kp3,
            Key::Kp4 => crate::core::Key::Kp4,
            Key::Kp5 => crate::core::Key::Kp5,
            Key::Kp6 => crate::core::Key::Kp6,
            Key::Kp7 => crate::core::Key::Kp7,
            Key::Kp8 => crate::core::Key::Kp8,
            Key::Kp9 => crate::core::Key::Kp9,
            Key::KpDelete => crate::core::Key::KpDelete,
            Key::Function => crate::core::Key::Function,
            Key::Unknown(_) => crate::core::Key::Unknown,
        }
    }
}
