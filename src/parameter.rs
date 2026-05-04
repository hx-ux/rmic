use std::default;

use rand::prelude::*;
use rand::rng;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    Int,
    Float,
    Choice,
    Bool,
    Separator,
    Link,
    Note,
    #[default]
    Unknown,
}

impl ParameterType {
    pub fn is_randomizable(&self) -> bool {
        matches!(self, Self::Int | Self::Float)
    }
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

    pub fn new(
        param_type: ParameterType,
        default: impl Into<String>,
        min: Option<String>,
        max: Option<String>,
        position: usize,
    ) -> Self {
        Self {
            param_type,
            default: default.into(),
            min,
            max,
            position,
        }
    }

    pub fn randomize(&mut self) {
        if !self.param_type.is_randomizable() {
            return;
        }

        let (min_val, max_val) = match (&self.min, &self.max) {
            (Some(min), Some(max)) => (min, max),
            _ => return,
        };

        let mut rng = rand::rng();

        match self.param_type {
            ParameterType::Int => {
                if let (Ok(min), Ok(max)) = (min_val.parse::<i32>(), max_val.parse::<i32>()) {
                    self.default = rng.random_range(min..=max).to_string();
                }
            }
            ParameterType::Float => {
                if let (Ok(min), Ok(max)) = (min_val.parse::<f32>(), max_val.parse::<f32>()) {
                    let result = rng.random_range(min..=max);

                    self.default = format!("{:.2}", result);
                }
            }
            ParameterType::Bool => {
                self.default = rng.random_range(0..=1).to_string();
            }
            ParameterType::Choice => {}
            _ => {}
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
