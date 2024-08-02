#![windows_subsystem = "windows"]

use window::initialize;
use config::load;

pub mod window;
pub mod action;
pub mod config;
pub mod paint;
pub mod event;
pub mod error;
pub mod util;
pub mod icon;

fn main() {
    unsafe { initialize(load());}
}
