use std::thread;
use std::time::Duration;

use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use sysinfo::{ProcessExt, SystemExt};

pub fn init_keyboard_interceptor() {
    unsafe {
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), None, 0);
        let mut msg = MSG::default();

        loop {
            GetMessageW(&mut msg, None, 0, 0);
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

unsafe extern "system" fn keyboard_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code >= 0 {
        let key_info = *(l_param.0 as *const KBDLLHOOKSTRUCT);

        dbg!(get_active_window_process_name());

        if key_info.vkCode == 'A' as u32 {
            // if w_param.0 == WM_KEYDOWN {
            return LRESULT(1);
            // }
        }
    }
    CallNextHookEx(None, n_code, w_param, l_param)
}

fn get_active_window_process_name() -> String {
    let hwnd = unsafe { GetForegroundWindow() };
    let mut process_id = 0;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut process_id)) };
    dbg!(process_id);

    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let process_name = system.process(sysinfo::Pid::from(process_id as usize)).map(|process| process.name().to_string());
    match process_name {
        Some(name) => name,
        None => "".to_string(),
    }
}
