use windows_sys::Win32::UI::WindowsAndMessaging::{ WA_INACTIVE, WM_ACTIVATE, WM_CLOSE, WM_DESTROY, WM_ERASEBKGND, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_PAINT };
use windows_sys::Win32::UI::WindowsAndMessaging::{ DefWindowProcW, DestroyWindow, PostQuitMessage };
use windows_sys::Win32::Foundation::{ HWND, LPARAM, LRESULT, WPARAM };
use crate::util::{handle_message, handle_message_read};
use windows_sys::Win32::Graphics::Gdi::InvalidateRect;
use crate::paint::wm_paint;
use crate::window::State;
use crate::util::loword;
use std::process::exit;
use crate::action;

pub unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => { PostQuitMessage(0); 0 }, WM_CLOSE => { DestroyWindow(hwnd); 0 },
        WM_LBUTTONDOWN => handle_message(|state| wm_mousedown(hwnd, wp, lp, state)),
        WM_MOUSEMOVE =>   handle_message(|state| wm_mousemove(hwnd, wp, lp, state)),
        WM_LBUTTONUP =>   handle_message(|state| wm_mouseup(hwnd, wp, lp, state)),
        WM_PAINT =>  handle_message_read(|state| wm_paint(hwnd, wp, lp, state)),
        WM_ACTIVATE => wm_activate(hwnd, wp, lp), WM_ERASEBKGND => 0,
        _ => DefWindowProcW(hwnd, msg, wp, lp),
    }
}

unsafe fn wm_activate(_hwnd: HWND, wp: WPARAM, _lp: LPARAM) -> LRESULT
{ if loword(wp) == WA_INACTIVE { exit(0); } 0 }
unsafe fn wm_mousemove(hwnd: HWND, _wp: WPARAM, lp: LPARAM, state: &mut State) {
    let mx=(lp&0xFFFF)as i32;let my=((lp>>16)&0xFFFF)as i32;
    let mut i=0;for icon in &mut state.icons{icon.hovered=mx
    >=(3+(i*44))&&mx<=(43+(i*44))&&my>=3&&my<=43;i+=1;}
    InvalidateRect(hwnd, std::ptr::null_mut(),1);
}
unsafe fn wm_mousedown(hwnd: HWND, _wp: WPARAM, lp: LPARAM, state: &mut State) {
    let mx =(lp&0xFFFF)as i32;let my=((lp>>16)&0xFFFF)as i32;
    let mut i=0;for icon in &mut state.icons{if mx>=(3+(i*44))&&
    mx<=(43+(i*44))&&my>=3&&my<=43{icon.pressed=true;}i+=1;}
    InvalidateRect(hwnd, std::ptr::null_mut(),1);
}
unsafe fn wm_mouseup(hwnd: HWND, _wp: WPARAM, _lp: LPARAM, state: &mut State) {
    for icon in &mut state.icons{if icon.pressed{action::
    execute(icon.action.clone());}icon.pressed=false;}
    InvalidateRect(hwnd, std::ptr::null_mut(),1);
}