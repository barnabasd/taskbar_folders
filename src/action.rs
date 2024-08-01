use std::{ffi::c_int, process::{exit, Command}, ptr::{null, null_mut}};
use windows_sys::{core::PCWSTR, Win32::Foundation::{HINSTANCE, HWND}};
use crate::util::to_wstring;

#[derive(Clone)]
pub struct Action { pub task: String, pub task_type: String, pub task_args: String, pub custom_icon: Option<String> }

pub fn execute(action: Action) {
    match &action.task_type as &str {
        "command" => { Command::new(action.task).arg(action.task_args); },
        "app" => { _ = Command::new(action.task).spawn(); },
        "website" => { open_url(&action.task as &str); },
        _ => {}
    }
    exit(0);
}

fn open_url(url: &str) -> bool {
    extern "system" {
        pub fn ShellExecuteW(
            hwnd: HWND, lpOperation: PCWSTR, lpFile: PCWSTR,
            lpParameters: PCWSTR, lpDirectory: PCWSTR,
            nShowCmd: c_int) -> HINSTANCE;
    }
    let action = to_wstring("open");
    let path = to_wstring(url);
    let result = unsafe {
        ShellExecuteW(null_mut(), action.as_ptr(), path.as_ptr(), null(), null(), 5 as c_int)
    };
    result as usize > 32
}