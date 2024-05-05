use bevy::prelude::Event;

use input_model::keyboard::{Key, KeyEvent};
use input_model::mouse::{Button, ButtonEvent};

use crate::input::InputEvent;

#[derive(Event, Debug)]
pub enum InputFilterEvent {
    Block(FilterInput),
    Unblock(FilterInput),
}

#[derive(Debug)]
pub enum FilterInput {
    Full,
    FullKeyboard,
    FullKeyboardPress,
    FullKeyboardRelease,
    FullMouse,
    Keyboard(FilterKeyEvent),
    MouseButton(FilterButtonEvent),
    MouseMove,
}

impl FilterInput {
    pub fn to_input_events(&self) -> Vec<InputEvent> {
        match self {
            FilterInput::Full => {
                let mut events = all_keyboard_events();
                events.append(&mut all_mouse_events());
                events
            }
            FilterInput::FullKeyboard => all_keyboard_events(),
            FilterInput::FullKeyboardPress => all_keyboard_press_events(),
            FilterInput::FullKeyboardRelease => all_keyboard_release_events(),
            FilterInput::FullMouse => all_mouse_events(),
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

fn all_mouse_events() -> Vec<InputEvent> {
    vec![
        InputEvent::MouseMove { x: 0., y: 0. },
        InputEvent::MouseButton(ButtonEvent::Pressed(Button::Left)),
        InputEvent::MouseButton(ButtonEvent::Released(Button::Left)),
        InputEvent::MouseButton(ButtonEvent::Pressed(Button::Right)),
        InputEvent::MouseButton(ButtonEvent::Released(Button::Right)),
        InputEvent::MouseButton(ButtonEvent::Pressed(Button::Middle)),
        InputEvent::MouseButton(ButtonEvent::Released(Button::Middle)),
        InputEvent::MouseButton(ButtonEvent::Pressed(Button::Unknown(0))), // todo how to block all definitely unknown input
        InputEvent::MouseButton(ButtonEvent::Released(Button::Unknown(0))),
    ]
}

fn all_keyboard_events() -> Vec<InputEvent> {
    let mut events = all_keyboard_press_events();
    events.append(&mut all_keyboard_release_events());
    events
}

fn all_keyboard_press_events() -> Vec<InputEvent> {
    vec![
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Alt)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::AltGr)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Backspace)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::CapsLock)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::ControlLeft)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::ControlRight)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Delete)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::DownArrow)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::End)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Escape)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F1)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F10)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F11)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F12)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F2)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F3)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F4)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F5)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F6)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F7)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F8)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::F9)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Home)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::LeftArrow)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::MetaLeft)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::MetaRight)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::PageDown)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::PageUp)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Return)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::RightArrow)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::ShiftLeft)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::ShiftRight)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Space)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Tab)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::UpArrow)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::PrintScreen)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::ScrollLock)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Pause)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::NumLock)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::BackQuote)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Num1)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Num2)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Num3)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Num4)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Num5)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Num6)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Num7)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Num8)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Num9)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Num0)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Minus)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Equal)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyQ)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyW)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyE)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyR)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyY)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyU)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyI)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyO)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyP)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::LeftBracket)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::RightBracket)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyA)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyS)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyD)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyF)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyG)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyH)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyJ)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyK)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyL)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::SemiColon)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Quote)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::BackSlash)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::IntlBackslash)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyZ)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyX)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyC)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyV)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyB)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyN)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyM)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Comma)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Dot)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Slash)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Insert)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KpReturn)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KpMinus)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KpPlus)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KpMultiply)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KpDivide)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Kp0)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Kp1)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Kp2)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Kp3)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Kp4)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Kp5)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Kp6)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Kp7)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Kp8)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Kp9)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KpDelete)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Function)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::Unknown(0))),
    ]
}

fn all_keyboard_release_events() -> Vec<InputEvent> {
    vec![
        InputEvent::Keyboard(KeyEvent::Released(Key::Alt)),
        InputEvent::Keyboard(KeyEvent::Released(Key::AltGr)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Backspace)),
        InputEvent::Keyboard(KeyEvent::Released(Key::CapsLock)),
        InputEvent::Keyboard(KeyEvent::Released(Key::ControlLeft)),
        InputEvent::Keyboard(KeyEvent::Released(Key::ControlRight)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Delete)),
        InputEvent::Keyboard(KeyEvent::Released(Key::DownArrow)),
        InputEvent::Keyboard(KeyEvent::Released(Key::End)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Escape)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F1)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F10)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F11)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F12)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F2)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F3)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F4)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F5)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F6)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F7)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F8)),
        InputEvent::Keyboard(KeyEvent::Released(Key::F9)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Home)),
        InputEvent::Keyboard(KeyEvent::Released(Key::LeftArrow)),
        InputEvent::Keyboard(KeyEvent::Released(Key::MetaLeft)),
        InputEvent::Keyboard(KeyEvent::Released(Key::MetaRight)),
        InputEvent::Keyboard(KeyEvent::Released(Key::PageDown)),
        InputEvent::Keyboard(KeyEvent::Released(Key::PageUp)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Return)),
        InputEvent::Keyboard(KeyEvent::Released(Key::RightArrow)),
        InputEvent::Keyboard(KeyEvent::Released(Key::ShiftLeft)),
        InputEvent::Keyboard(KeyEvent::Released(Key::ShiftRight)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Space)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Tab)),
        InputEvent::Keyboard(KeyEvent::Released(Key::UpArrow)),
        InputEvent::Keyboard(KeyEvent::Released(Key::PrintScreen)),
        InputEvent::Keyboard(KeyEvent::Released(Key::ScrollLock)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Pause)),
        InputEvent::Keyboard(KeyEvent::Released(Key::NumLock)),
        InputEvent::Keyboard(KeyEvent::Released(Key::BackQuote)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Num1)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Num2)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Num3)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Num4)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Num5)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Num6)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Num7)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Num8)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Num9)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Num0)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Minus)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Equal)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyQ)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyW)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyE)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyR)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyT)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyY)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyU)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyI)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyO)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyP)),
        InputEvent::Keyboard(KeyEvent::Released(Key::LeftBracket)),
        InputEvent::Keyboard(KeyEvent::Released(Key::RightBracket)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyA)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyS)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyD)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyF)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyG)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyH)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyJ)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyK)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyL)),
        InputEvent::Keyboard(KeyEvent::Released(Key::SemiColon)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Quote)),
        InputEvent::Keyboard(KeyEvent::Released(Key::BackSlash)),
        InputEvent::Keyboard(KeyEvent::Released(Key::IntlBackslash)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyZ)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyX)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyC)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyV)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyB)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyN)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyM)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Comma)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Dot)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Slash)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Insert)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KpReturn)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KpMinus)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KpPlus)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KpMultiply)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KpDivide)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Kp0)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Kp1)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Kp2)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Kp3)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Kp4)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Kp5)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Kp6)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Kp7)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Kp8)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Kp9)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KpDelete)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Function)),
        InputEvent::Keyboard(KeyEvent::Released(Key::Unknown(0))),
    ]
}