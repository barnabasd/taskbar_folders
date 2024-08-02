use std::result::Result::Ok;
use crate::window::State;
use std::fs::File;

pub fn load() -> State {
    let _file = File::open("config.json");

    if let Ok(file) = _file {
        let state: State = serde_json::from_reader(file).unwrap();
        return state;
    }
    else {
        _ = File::create("config.json");
        return vec![];
    }
}