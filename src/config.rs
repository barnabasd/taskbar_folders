use crate::window::State;
use std::fs::File;

pub fn load() -> State {
    let file = File::open("config.json").unwrap();
    let state: State = serde_json::from_reader(file).unwrap();
    state
}