use rand::prelude::*;
use serde::Deserialize;
use strum_macros::Display;

#[derive(Debug, Clone, Deserialize, PartialEq, Default, Display)]
#[serde(rename_all = "lowercase")]
#[allow(unused_attributes)]
pub enum ParameterType {
    #[strum(to_string = "int")]
    Int,
    #[strum(to_string = "float")]
    Float,
    #[strum(to_string = "choice")]
    Choice,
    #[strum(to_string = "bool")]
    Bool,
    #[strum(to_string = "value")]
    Value,
    #[strum(to_string = "point")]
    Point,
    #[strum(to_string = "separator")]
    Separator,
    #[strum(to_string = "link")]
    Link,
    #[strum(to_string = "note")]
    Note,
    #[strum(to_string = "color")]
    Color,
    #[strum(to_string = "text")]
    Text,
    #[strum(to_string = "button")]
    Button,

    #[strum(to_string = "file")]
    File,
    #[strum(to_string = "folder")]
    Folder,
    #[default]
    #[strum(to_string = "unknown")]
    Unknown,
}

impl ParameterType {
    pub fn is_data_type(&self) -> bool {
        let type_check = matches!(
            self,
            Self::Int
                | Self::Float
                | Self::Bool
                | Self::Choice
                | Self::Color
                | Self::Text
                | Self::Value
                | Self::Point
        );
        type_check
    }
}

/// A parameter for a G'MIC filter.
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Data type (e.g., "float", "int", "bool", "color").
    pub param_type: ParameterType,
    /// Descriptive name (optional).
    pub name: Option<String>,
    /// Default value.
    pub default: String,
    /// Minimum value (optional).
    pub min: Option<String>,
    /// Maximum value (optional).
    pub max: Option<String>,
    /// Position in the parameter list.
    pub position: usize,
}

/// Single Paramter of an GMIC Filter
impl Parameter {
    pub fn const_value(value: String, position: usize) -> Self {
        Self {
            param_type: Self::infer_param_type(value.clone()),
            default: value,
            min: None,
            max: None,
            name: None,
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
        let mut default_str = default.into();
        //  these values are written as 0,"value" in the cli
        if matches!(param_type, ParameterType::Text | ParameterType::Value) {
            if !default_str.starts_with('"') && !default_str.ends_with('"') {
                default_str = format!("\"{}\"", default_str);
            }
        }

        Self {
            param_type,
            default: default_str,
            name: None,
            min,
            max,
            position,
        }
    }

    pub fn randomize(&mut self) {
        if !self.param_type.is_data_type() {
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
            ParameterType::Color => {
                let r = rng.random_range(0..=255).to_string();
                let g = rng.random_range(0..=255).to_string();
                let b = rng.random_range(0..=255).to_string();
                self.default = format!("{},{},{}", r, g, b)
            }
            ParameterType::Choice => {
                if let (Ok(min), Ok(max)) = (min_val.parse::<i32>(), max_val.parse::<i32>()) {
                    self.default = rng.random_range(min..=max).to_string();
                }
            }
            // Point can be from -100 to +100
            ParameterType::Point => {
                let x = rng.random_range(-100..=100).to_string();
                let y = rng.random_range(-100..=100).to_string();
                self.default = format!("{},{}", x, y)
            }
            _ => {
                return;
            }
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
