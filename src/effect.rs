use crate::parameter::{self, Parameter, ParameterType};
use serde::{Deserialize, Deserializer};
/// Represents a G'MIC effect
#[derive(Debug, Clone, Deserialize)]
pub struct Effect {
    /// The G'MIC command name.
    pub command: String,

    /// List of parameters and other elements.
    #[serde(deserialize_with = "deserialize_parameters")]
    pub parameters: Vec<Parameter>,

    /// a raw string with all the commands in it
    #[serde(skip, default)]
    raw: bool,
}

impl Effect {
    pub fn new(command: String, params: Vec<Parameter>) -> Self {
        Self {
            command: command.clone(),
            parameters: params,
            raw: false,
        }
    }

    pub fn new_raw(command: String) -> Self {
        Self {
            command,
            parameters: vec![],
            raw: true,
        }
    }

    /// Returns ["command", "value1,value2,..."] or the raw string
    pub fn forargs(&self) -> Vec<String> {
        if self.raw {
            return self
                .command
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
        }

        let params = self
            .parameters
            .iter()
            .map(|p| p.default.clone())
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
        name: Option<String>,
        min: Option<String>,
        max: Option<String>,
        pos: Option<String>,
    }

    let helpers: Vec<ParamHelper> = Vec::deserialize(deserializer)?;
    let mut parameters = Vec::new();
    for helper in helpers {
        match helper.param_type {
            ParameterType::Int
            | ParameterType::Float
            | ParameterType::Bool
            | ParameterType::Choice => {
                let position = helper
                    .pos
                    .as_ref()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);

                parameters.push(Parameter {
                    param_type: helper.param_type,
                    default: helper.default.unwrap_or_default(),
                    min: helper.min,
                    max: helper.max,
                    position,
                });
            }
            _ => {}
        }
    }
    Ok(parameters)
}
