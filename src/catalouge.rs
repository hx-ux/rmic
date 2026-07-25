use std::{fs::read_to_string, path::PathBuf};

use crate::{GmicError, ParameterType, filter::Filter};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    pub name: String,
    pub filters: Vec<Filter>,
}

#[derive(Deserialize, Debug, Clone)]
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

    pub fn find_filter(&self, query: &str) -> Option<&Filter> {
        for category in &self.categories {
            for filter in &category.filters {
                if filter.command == query {
                    return Some(filter);
                }
            }
        }
        None
    }

    pub fn deserialize(json_data: &str) -> Result<Self, GmicError> {
        serde_json::from_str(json_data).map_err(|_| GmicError::JsonParseError)
    }

    pub fn exlusion_list_cli() -> (Vec<String>, Vec<ParameterType>) {
        let exclude_commands = vec!["fx_blend".to_string(), "fx_transfer_pca".to_string()];
        let exclude_params_type: Vec<ParameterType> = vec![];

        return (exclude_commands, exclude_params_type);
    }
}
