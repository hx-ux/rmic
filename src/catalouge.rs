use std::{fs::read_to_string, path::PathBuf};

use crate::{GmicError, filter::Filter};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Category {
    pub name: String,
    pub filters: Vec<Filter>,
}

#[derive(Deserialize, Debug)]
pub struct Catalouge {
    pub gmic_version: String,
    pub categories: Vec<Category>,
}

impl Catalouge {
    pub fn load_local(path: &PathBuf) -> Result<Self, GmicError> {
        match read_to_string(&path) {
            Ok(data) => match Self::deserialize(&data) {
                Ok(result) => Ok(result),
                Err(_) => Err(GmicError::JsonParseError),
            },
            Err(err) => return Err(GmicError::Io(err)),
        }
    }

    pub fn deserialize(json_data: &str) -> Result<Self, GmicError> {
        serde_json::from_str(json_data).map_err(|_| GmicError::JsonParseError)
    }
}
