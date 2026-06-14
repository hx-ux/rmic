use std::{
    fs::{create_dir, read_to_string},
    path::PathBuf,
};

use crate::{GmicError, filter::Filter};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Category {
    pub name: String,
    pub filters: Vec<Filter>,
}

#[derive(Deserialize, Debug)]
pub struct FilterList {
    pub gmic_version: String,
    pub categories: Vec<Category>,
}

impl FilterList {
    pub fn load_remote() -> Result<Self, GmicError> {
        let mut path = PathBuf::new();

        if let Some(home) = dirs::home_dir() {
            path.push(home);
        } else {
            return Err(GmicError::GmicNotFound);
        }

        path.push(".config/rmic/");

        match create_dir(path) {
            Ok(_) => {}
            Err(_) => {}
        }

        return Err(GmicError::GmicNotFound);
    }

    pub fn load_local(path: &PathBuf) -> Result<Self, GmicError> {
        let json_data = match read_to_string(&path) {
            Ok(data) => data,
            Err(_) => return Err(GmicError::JsonParseError),
        };

        Self::deserialize(&json_data)
    }

    pub fn deserialize(json_data: &str) -> Result<Self, GmicError> {
        serde_json::from_str(json_data).map_err(|_| GmicError::JsonParseError)
    }
}
