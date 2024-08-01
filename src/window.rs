use windows_sys::Win32::UI::WindowsAndMessaging::{ CreateWindowExW, DispatchMessageW, GetMessageW, LoadCursorW, RegisterClassW, ShowWindow, TranslateMessage };
use windows_sys::Win32::Graphics::GdiPlus::{ GdiplusShutdown, GdiplusStartup, GdiplusStartupInput, GdiplusStartupOutput };
use windows_sys::Win32::UI::WindowsAndMessaging::{ IDC_ARROW, SW_SHOW, WS_POPUP, MSG, WNDCLASSW };
use windows_sys::Win32::Graphics::Gdi::{ CreateRoundRectRgn, SetWindowRgn };
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use crate::util::{ get_startup_loc, to_wstring };
use windows_sys::Win32::Foundation::TRUE;
use lazy_static::lazy_static;
use crate::action::Action;
use std::ptr::null_mut;
use std::sync::RwLock;

lazy_static! { pub static ref APP_STATE: RwLock<Option<State>> = RwLock::new(None); }

pub struct TaskbarIcon { pub hovered: bool, pub pressed: bool, pub action: Action }
pub struct State { pub icons: Vec<TaskbarIcon> }

pub unsafe fn initialize(icons: State) {
    let width = 2 + (icons.icons.len() * 44);
    let pos = get_startup_loc(width as i32, 47, 5);
    let mut gdiplus_token = 0;
    let startup_input = GdiplusStartupInput {
        GdiplusVersion: 1,
        DebugEventCallback: 0,
        SuppressBackgroundThread: 0,
        SuppressExternalCodecs: 0
    };
    let mut startup_output = GdiplusStartupOutput { NotificationHook: 0, NotificationUnhook: 0 };
    unsafe { GdiplusStartup(&mut gdiplus_token, &startup_input, &mut startup_output) };
    RegisterClassW(&WNDCLASSW {
        style: 0, lpfnWndProc: Some(crate::event::window_proc),
        hbrBackground: null_mut(), lpszMenuName: null_mut(),
        cbClsExtra: 0, cbWndExtra: 0, hIcon: null_mut(),
        lpszClassName: to_wstring("window").as_ptr(),
        hCursor: LoadCursorW(null_mut(), IDC_ARROW),
        hInstance: GetModuleHandleW(null_mut()),
    });
    *APP_STATE.write().unwrap() = Some(icons);  
    let hwnd = CreateWindowExW(
        0, to_wstring("window").as_ptr(),
        to_wstring("").as_ptr(),
        WS_POPUP,
        pos.0, pos.1, width as i32, 47, null_mut(),
        null_mut(),
        GetModuleHandleW(null_mut()),
        null_mut(),
    );
    SetWindowRgn(hwnd, CreateRoundRectRgn(0, 0, width as i32, 47, 16, 16), TRUE);
    ShowWindow(hwnd, SW_SHOW);
    let mut msg: MSG = std::mem::zeroed();
    while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    GdiplusShutdown(gdiplus_token);
}