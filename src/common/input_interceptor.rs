// This module heavily inspired by rdev crate: https://crates.io/crates/rdev

use std::os::raw::c_int;
use std::ptr::null_mut;
use std::sync::Mutex;
use winapi::shared::minwindef::{DWORD, LPARAM, LRESULT, WPARAM};
use winapi::um::winuser::{CallNextHookEx, GetMessageA, HC_ACTION, KBDLLHOOKSTRUCT, SetWindowsHookExA, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYUP, WM_SYSKEYDOWN};
use crate::core::{Key, Event};
use winapi::shared::minwindef::WORD;
use winapi::shared::windef::HHOOK;

type RawCallback = unsafe extern "system" fn(code: c_int, param: WPARAM, lpdata: LPARAM) -> LRESULT;

static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(Event)>> = None;
static mut HOOK: HHOOK = null_mut();
static INPUT_FILTERS: Mutex<Vec<Filter>> = Mutex::new(Vec::new());

#[derive(Debug, Eq, PartialEq)]
pub enum Filter {
    BlockAll,
    BlockList(Vec<Event>),
    ExcludeList(Vec<Event>),
}

impl Filter {
    fn is_blocked(&self, event: &Event) -> bool {
        match self {
            Filter::BlockAll => true,
            Filter::BlockList(events) => events.contains(event),
            Filter::ExcludeList(events) => !events.contains(event),
        }
    }
}

pub fn listen<T>(callback: T)
    where
        T: FnMut(Event) + 'static,
{
    unsafe {
        GLOBAL_CALLBACK = Some(Box::new(callback));
        set_key_hook(raw_callback);

        GetMessageA(null_mut(), null_mut(), 0, 0);
    }
}

pub fn filter(filter: Filter) {
    let mut filters = INPUT_FILTERS.lock()
        .expect("Input filters already locked");

    if !filters.contains(&filter){
        filters.push(filter);
    }
}

pub fn remove_filter(filter: Filter) {
    let mut filters = INPUT_FILTERS.lock()
        .expect("Input filters already locked");

    if let Some(position) = filters.iter().position(|f| f.eq(&filter)) {
        filters.remove(position);
    }
}

unsafe fn set_key_hook(callback: RawCallback) {
    let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(callback), null_mut(), 0);
    HOOK = hook;
}

unsafe extern "system" fn raw_callback(code: c_int, param: WPARAM, lpdata: LPARAM) -> LRESULT {
    if code == HC_ACTION {
        let opt = convert(param, lpdata);
        if let Some(event) = opt {
            if let Some(callback) = &mut GLOBAL_CALLBACK {

                let blocked = INPUT_FILTERS.lock()
                    .expect("Input filters already locked")
                    .iter()
                    .any(|filter| filter.is_blocked(&event));

                callback(event);
                if blocked { return 1; }
            }
        }
    }
    CallNextHookEx(HOOK, code, param, lpdata)
}

pub unsafe fn convert(param: WPARAM, lpdata: LPARAM) -> Option<Event> {
    match param.try_into() {
        Ok(WM_KEYDOWN) | Ok(WM_SYSKEYDOWN) => {
            let code = get_code(lpdata);
            let key = key_from_code(code as u16);
            Some(Event::KeyPress(key))
        }
        Ok(WM_KEYUP) | Ok(WM_SYSKEYUP) => {
            let code = get_code(lpdata);
            let key = key_from_code(code as u16);
            Some(Event::KeyRelease(key))
        }
        _ => None,
    }
}

pub unsafe fn get_code(lpdata: LPARAM) -> DWORD {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);
    kb.vkCode
}

macro_rules! decl_keycodes {
    ($($key:ident, $code:literal),*) => {
        //TODO: make const when rust lang issue #49146 is fixed
        pub fn code_from_key(key: Key) -> Option<WORD> {
            match key {
                $(
                    Key::$key => Some($code),
                )*
                _ => None,
            }
        }

        //TODO: make const when rust lang issue #49146 is fixed
        pub fn key_from_code(code: WORD) -> Key {
            match code {
                $(
                    $code => Key::$key,
                )*
                _ => Key::Unknown
            }
        }
    };
}

decl_keycodes! {
    AltLeft, 164,
    AltRight, 165,
    Backspace, 0x08,
    CapsLock, 20,
    ControlLeft, 162,
    ControlRight, 163,
    Delete, 46,
    DownArrow, 40,
    End, 35,
    Escape, 27,
    F1, 112,
    F10, 121,
    F11, 122,
    F12, 123,
    F2, 113,
    F3, 114,
    F4, 115,
    F5, 116,
    F6, 117,
    F7, 118,
    F8, 119,
    F9, 120,
    Home, 36,
    LeftArrow, 37,
    MetaLeft, 91,
    PageDown, 34,
    PageUp, 33,
    Return, 0x0D,
    RightArrow, 39,
    ShiftLeft, 160,
    ShiftRight, 161,
    Space, 32,
    Tab, 0x09,
    UpArrow, 38,
    PrintScreen, 44,
    ScrollLock, 145,
    Pause, 19,
    NumLock, 144,
    BackQuote, 192,
    Num1, 49,
    Num2, 50,
    Num3, 51,
    Num4, 52,
    Num5, 53,
    Num6, 54,
    Num7, 55,
    Num8, 56,
    Num9, 57,
    Num0, 48,
    Minus, 189,
    Equal, 187,
    KeyQ, 81,
    KeyW, 87,
    KeyE, 69,
    KeyR, 82,
    KeyT, 84,
    KeyY, 89,
    KeyU, 85,
    KeyI, 73,
    KeyO, 79,
    KeyP, 80,
    LeftBracket, 219,
    RightBracket, 221,
    KeyA, 65,
    KeyS, 83,
    KeyD, 68,
    KeyF, 70,
    KeyG, 71,
    KeyH, 72,
    KeyJ, 74,
    KeyK, 75,
    KeyL, 76,
    SemiColon, 186,
    Quote, 222,
    BackSlash, 220,
    IntlBackslash, 226,
    KeyZ, 90,
    KeyX, 88,
    KeyC, 67,
    KeyV, 86,
    KeyB, 66,
    KeyN, 78,
    KeyM, 77,
    Comma, 188,
    Dot, 190,
    Slash, 191,
    Insert, 45,
    //KP_RETURN, 13,
    KpMinus, 109,
    KpPlus, 107,
    KpMultiply, 106,
    KpDivide, 111,
    Kp0, 96,
    Kp1, 97,
    Kp2, 98,
    Kp3, 99,
    Kp4, 100,
    Kp5, 101,
    Kp6, 102,
    Kp7, 103,
    Kp8, 104,
    Kp9, 105,
    KpDelete, 110
}
