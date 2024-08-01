use windows_sys::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromPoint, HMONITOR, MONITORINFO, MONITOR_DEFAULTTONEAREST};
use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;
use windows_sys::Win32::Foundation::{LRESULT, POINT, WPARAM};
use std::{ffi::OsStr, os::windows::ffi::OsStrExt};
use crate::window::{InternalIconState, State, APP_STATE};

pub fn to_wstring(str: &str) -> Vec<u16> { OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect() }
pub unsafe fn loword(value: WPARAM) -> u32 { value as u32 & 0xFFFF }

pub fn handle_message<F>(f: F) -> LRESULT 
where F: FnOnce(&mut State, &mut InternalIconState) -> () {
    let mut app_state = APP_STATE.write().unwrap();
    if let Some(ref mut state) = *app_state { f(&mut state.0, &mut state.1); } 0
}

pub fn handle_message_read<F>(f: F) -> LRESULT 
where F: FnOnce(&State, InternalIconState) -> () {
    let app_state = APP_STATE.read().unwrap();
    if let Some(ref state) = *app_state { f(&state.0, state.1.clone()); } 0
}

const TASKBAR_HEIGHT: i32 = 47;

pub fn get_startup_loc(w: i32, h: i32, h_pad: i32) -> (i32, i32) {
    unsafe {
        let mut cursor_pos: POINT = POINT { x: 0, y: 0 };
        if GetCursorPos(&mut cursor_pos) == 0 { return (500, 500); }
        let monitor: HMONITOR = MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONEAREST);
        if monitor.is_null() { return (500, 500); }
        let mut monitor_info: MONITORINFO = std::mem::zeroed();
        monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        if GetMonitorInfoW(monitor, &mut monitor_info) == 0 { return (500, 500); }
        let mw = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
        let sp_t = monitor_info.rcMonitor.bottom - (TASKBAR_HEIGHT + h + h_pad);
        let sp_l = (mw / 2) - (w / 2);
        (sp_l, sp_t)
    }
}