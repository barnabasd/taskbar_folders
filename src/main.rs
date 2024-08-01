//#![windows_subsystem = "windows"]

use window::{initialize, State, TaskbarIcon};
use crate::action::Action;

pub mod window;
pub mod action;
pub mod paint;
pub mod event;
pub mod util;
pub mod icon;

fn main() {
    unsafe { initialize(State { icons: vec![
        TaskbarIcon {
            hovered: false, pressed: false,
            action: Action {
                task: "D:\\exe.exe".to_string(),
                task_args: "".to_string(),
                task_type: "app".to_string()
            }
        },
        TaskbarIcon {
            hovered: false, pressed: false,
            action: Action {
                task: "C:\\Users\\Barnabás\\AppData\\Local\\BraveSoftware\\Brave-Browser\\Application\\brave.exe".to_string(),
                task_args: "".to_string(),
                task_type: "app".to_string()
            }
        },
        TaskbarIcon {
            hovered: false, pressed: false,
            action: Action {
                task: "https://www.google.com".to_string(),
                task_args: "".to_string(),
                task_type: "website".to_string()
            }
        },
        TaskbarIcon {
            hovered: false, pressed: false,
            action: Action {
                task: "D:\\exe.exe".to_string(),
                task_args: "".to_string(),
                task_type: "app".to_string()
            }
        },
        TaskbarIcon {
            hovered: false, pressed: false,
            action: Action {
                task: "D:\\exe.exe".to_string(),
                task_args: "".to_string(),
                task_type: "app".to_string()
            }
        },
        TaskbarIcon {
            hovered: false, pressed: false,
            action: Action {
                task: "D:\\exe.exe".to_string(),
                task_args: "".to_string(),
                task_type: "app".to_string()
            }
        }
    ]});}
}
