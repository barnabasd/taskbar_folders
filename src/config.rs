use std::{io::Write, result::Result::Ok};
use crate::{error::error, window::State};
use std::fs::File;

pub fn load() -> State {
    let _file = File::open("config.json");

    if let Ok(file) = _file {
        if let Ok(state) = serde_json::from_reader(file) as Result<State, _> {
            if state.len() == 0 {
                error(crate::error::ErrorType::ConfigFileIconListEmpty);
            }
            return state;
        }
        else {
            error(crate::error::ErrorType::CouldntParseConfigFile);
            return vec![];
        }
    }
    else {
        let _config = File::create("config.json");
        if let Ok(mut config) = _config {
            _ = config.write_all(b"[]");
        }
        else {
            error(crate::error::ErrorType::CouldntCreateMissingConfigFile);
        }
        return vec![];
    }
}