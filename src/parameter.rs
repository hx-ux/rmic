use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    Int,
    Float,
    Choice,
    Bool,
    Separator,
    Link,
    Note,
    Unknown,
}

/// A parameter for a G'MIC effect.
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Data type (e.g., "float", "int", "bool", "color").
    pub param_type: ParameterType,
    /// Descriptive name (optional).
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
            default: value,
            min: None,
            max: None,
            position,
        }
    }

    fn infer_param_type(value: String) -> ParameterType {
        if value.parse::<i32>().is_ok() {
            return ParameterType::Int;
        } else if value.parse::<f32>().is_ok() {
            return ParameterType::Float;
        } else if value.parse::<bool>().is_ok() {
            return ParameterType::Bool;
        }
        ParameterType::Choice
    }
}
