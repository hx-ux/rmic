use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
enum ParameterType {
    int,
    float,
    choice,
    bool,
    seperator,
    link,
    note,
    unknwon,
}

/// A parameter for a G'MIC effect.
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Data type (e.g., "float", "int", "bool", "color").
    pub param_type: ParameterType,
    /// Descriptive name (optional).
    pub name: Option<String>,
    // pub command: String,
    /// Default value.
    pub default: String,
    /// Minimum value (optional).
    pub min: Option<String>,
    /// Maximum value (optional).
    pub max: Option<String>,
    /// Position in the parameter list.
    pub position: usize,
}

impl Parameter {
    pub fn const_value(value: String, position: usize) -> Self {
        Self {
            param_type: Self::infer_param_type(value.clone()),
            name: None,
            default: value,
            min: None,
            max: None,
            position,
        }
    }

    fn infer_param_type(value: String) -> ParameterType {
        ParameterType::unknwon
    }
}
