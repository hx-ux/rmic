use crate::parameter::{Parameter, ParameterChoice, ParameterType};
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;
/// Represents a G'MIC Filter
#[derive(Debug, Clone, Deserialize)]
pub struct Filter {
    /// The G'MIC command name.
    pub command: String,
    /// List of parameters and other elements.
    #[serde(deserialize_with = "deserialize_parameters")]
    pub parameters: Vec<Parameter>,
    /// a raw string with all the commands in it
    #[serde(skip, default)]
    raw_filter: bool,
    /// name of the effect
    pub name: Option<String>,
}

impl Filter {
    pub fn new(command: String, params: Vec<Parameter>) -> Self {
        Self {
            command: command.clone(),
            parameters: params,
            raw_filter: false,
            name: None,
        }
    }

    pub fn new_raw(command: String) -> Self {
        Self {
            command,
            parameters: vec![],
            raw_filter: true,
            name: None,
        }
    }

    pub fn new_params(command: String, parameters: Vec<Parameter>) -> Self {
        Self {
            command,
            parameters,
            raw_filter: false,
            name: None,
        }
    }

    /// Returns ["command", "value1,value2,..."] or the raw string.
    /// Skips commands that are "_none" (no-op/placeholder in G'MIC definitions).
    pub fn to_cli_command(&self) -> Vec<String> {
        if self.command == "_none_" {
            return vec![];
        }
        if self.raw_filter {
            return self
                .command
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
        }

        let params = self
            .parameters
            .iter()
            .filter(|p| p.param_type.is_data_type())
            .map(|p| p.cli_value())
            .collect::<Vec<_>>()
            .join(",");

        vec![self.command.clone(), params]
    }

    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn randomize(&mut self) {
        for p in self.parameters.iter_mut() {
            p.randomize();
        }
    }
    // check if the filter contains and matches any of this
    pub fn has_property(
        &self,
        command: Option<Vec<String>>,
        param_type: Option<Vec<ParameterType>>,
    ) -> bool {
        if let Some(commands) = command {
            if commands.contains(&self.command) {
                return true;
            }
        }

        if let Some(types) = param_type {
            return self
                .parameters
                .iter()
                .any(|d| types.contains(&d.param_type));
        }
        return false;
    }
}

fn deserialize_parameters<'de, D>(deserializer: D) -> Result<Vec<Parameter>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct ParamHelper {
        #[serde(rename = "type")]
        param_type: ParameterType,
        default: Option<String>,
        // fallback for default if type == point
        position: Option<String>,
        // fallback for default if type == value
        value: Option<String>,
        name: Option<String>,
        min: Option<String>,
        max: Option<String>,
        pos: Option<String>,
        #[serde(default)]
        choices: Option<BTreeMap<String, String>>,
    }

    let helpers: Vec<ParamHelper> = Vec::deserialize(deserializer)?;
    let mut parameters = Vec::new();

    for helper in helpers {
        let position = helper
            .pos
            .as_ref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let mut default: Option<String> = None;
        if let Some(val) = helper.default {
            default = Some(val);
        } else if let Some(val) = helper.position {
            default = Some(val);
        } else if let Some(val) = helper.value {
            default = Some(val);
        }

        let choices = helper.choices.map(|m| {
            m.into_iter()
                .map(|(value, label)| ParameterChoice { value, label })
                .collect()
        });

        if let Some(default_val) = default {
            parameters.push(Parameter {
                param_type: helper.param_type,
                default: default_val,
                min: helper.min,
                max: helper.max,
                position,
                name: helper.name,
                choices,
            });
        }
    }

    Ok(parameters)
}
