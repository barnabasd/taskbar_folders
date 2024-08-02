use std::process::exit;

use win_msgbox::Okay;

    pub enum ErrorType {
    CouldntCreateMissingConfigFile,
    ConfigFileIconListEmpty,
    CouldntParseConfigFile
}

pub fn error(error_type: ErrorType) {
    match error_type {
        ErrorType::CouldntCreateMissingConfigFile => {
            _ = win_msgbox::error::<Okay>("Couldn't find config.json.\nAnd while trying to create the file\nan error occured.").title("Error Loading Config").show().unwrap();
        },
        ErrorType::ConfigFileIconListEmpty => {
            _ = win_msgbox::error::<Okay>("The list of icons specified in config.json is empty.").title("Error Loading Icons").show().unwrap();
        },
        ErrorType::CouldntParseConfigFile => {
            _ = win_msgbox::error::<Okay>("Couldn't parse config.json correctly.").title("Error Loading Config").show().unwrap();
        }
    }
    exit(0);
}