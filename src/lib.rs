//! # RMIC: Rust G'MIC Interface
//!
//! This library provides a builder pattern for constructing and executing G'MIC commands.
//! It allows chaining image processing effects, setting input/output files, and executing via the CLI.

use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};

/// Errors that can occur during G'MIC operations.
#[derive(Debug)]
pub enum GmicError {
    /// I/O error (e.g., file access).
    Io(io::Error),
    /// Command execution failed, with stderr output.
    ExecutionFailed(String),
    /// Input file not found.
    InputNotFound,
    /// G'MIC binary not found.
    BinNotFound,
}

impl From<io::Error> for GmicError {
    fn from(err: io::Error) -> Self {
        GmicError::Io(err)
    }
}

/// A parameter for a G'MIC effect.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Parameter {
    /// Data type (e.g., "float", "int", "bool", "color").
    // #[serde(rename = "type")]
    // pub param_type: String,
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
            name: None,
            default: value,
            min: None,
            max: None,
            position,
        }
    }
}

/// Represents a G'MIC effect with metadata.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GmicEffect {
    /// Short name of the effect (optional).
    pub name: Option<String>,
    /// The G'MIC command name.
    pub command: String,
    /// The preview command.
    // pub command_preview: String,
    /// List of parameters and other elements.
    pub parameters: Vec<Parameter>,
    /// a raw string with all the commands in it
    raw: bool,
}

impl GmicEffect {
    pub fn new(command: String, params: Vec<Parameter>) -> Self {
        Self {
            name: None,
            command: command.clone(),
            parameters: params,
            raw: false,
        }
    }

    pub fn new_raw(command: String) -> Self {
        Self {
            name: None,
            command,
            parameters: vec![],
            raw: true,
        }
    }

    /// Gets the of the effect parameters
    pub fn get_parameters(&self) -> Vec<&Parameter> {
        self.parameters.iter().collect()
    }

    pub fn to_stri(&self) -> String {
        if self.raw {
            return self.command.clone();
        }

        let params: String = self
            .get_parameters()
            .iter()
            .map(|p| p.default.clone())
            .collect::<Vec<_>>()
            .join(",");

        // params
        format!("{} {}", self.command.clone(), params)
    }

    /// Returns ["command", "value1,value2,..."] or the raw string
    pub fn forargs(&self) -> Vec<String> {
        if self.raw {
            return vec![self.command.clone()];
        }

        let params: String = self
            .get_parameters()
            .iter()
            .map(|p| p.default.clone())
            .collect::<Vec<_>>()
            .join(",");

        vec![self.command.clone(), params]
    }
}

/// Builder for G'MIC commands.
pub struct Gmic {
    binary: String,
    effects: Vec<GmicEffect>,
    input_file: Option<PathBuf>,
    output_file: Option<PathBuf>,
}

impl Default for Gmic {
    fn default() -> Self {
        Self {
            binary: "gmic".to_string(),
            effects: Vec::new(),
            input_file: None,
            output_file: None,
        }
    }
}

impl Gmic {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new G'MIC builder with a custom binary path.
    pub fn with_binary(path: &str) -> Self {
        Self {
            binary: path.to_string(),
            ..Self::default()
        }
    }

    /// Sets the input file.
    pub fn input<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.input_file = Some(path.as_ref().to_path_buf());
        self
    }

    /// Sets the output file.
    pub fn output<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.output_file = Some(path.as_ref().to_path_buf());
        self
    }

    /// Adds a effect with params
    pub fn add_effect(mut self, command: &str, params: &[&str]) -> Self {
        let parameters: Vec<Parameter> = params
            .iter()
            .enumerate()
            .map(|(idx, value)| Parameter::const_value(value.to_string(), idx))
            .collect();

        let effect = GmicEffect::new(command.to_string(), parameters);
        self.effects.push(effect);
        self
    }

    /// Adds a parsed effect with custom values.
    ///
    /// Values should be in the order of the effect's parameters.
    pub fn add_parsed_effect(mut self, effect: &GmicEffect, values: &[&str]) -> Self {
        // let values = values.iter().map(|s| s.to_string()).collect();
        // self.effect_args.push(EffectInstance {
        //     effect: effect.clone(),
        //     values,
        // });
        self
    }

    /// Adds raw arguments.
    pub fn add_raw_arg(mut self, arg: &str) -> Self {
        if !arg.is_empty() {
            let effect = GmicEffect::new_raw(arg.to_string());

            self.effects.push(effect);
        }
        self
    }

    fn generate_command(&self) -> Command {
        let mut command = Command::new(&self.binary);

        if let Some(ref input) = self.input_file {
            command.arg("-input").arg(input);
        }

        for effect in &self.effects {
            command.args(effect.forargs());
        }

        if let Some(ref output) = self.output_file {
            command.arg("-output").arg(output);
        }

        command
    }
    /// Executes the built command.
    pub fn execute(&self) -> Result<(), GmicError> {
        if let Some(ref input) = self.input_file {
            if !input.exists() {
                return Err(GmicError::InputNotFound);
            }
        }

        let output = self.generate_command().output()?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(GmicError::ExecutionFailed(stderr.to_string()))
        }
    }

    /// Previews the command.
    pub fn dry_run(&self) -> String {
        self.generate_command()
            .get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Loads G'MIC effects from a JSON file.
///
/// The JSON should be an array of effect objects.
pub fn load_effects_from_json<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<GmicEffect>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let effects: Vec<GmicEffect> = serde_json::from_str(&data)?;
    Ok(effects)
}

// Utility methods remain similar, but updated to use add_command

impl Gmic {
    pub fn to_rgba(self) -> Self {
        self.add_raw_arg("to_rgba")
    }

    pub fn to_gray(self) -> Self {
        self.add_raw_arg("to_gray")
    }

    pub fn solarize(self) -> Self {
        self.add_raw_arg("solarize")
    }

    pub fn rotate(self, degree: u16) -> Self {
        self.add_effect("rotate", &[&degree.to_string()])
    }

    pub fn blur(self, radius: f32) -> Self {
        self.add_effect("blur", &[&radius.to_string()])
    }

    pub fn resize(self, width: u32, height: u32) -> Self {
        self.add_effect("resize", &[&width.to_string(), &height.to_string()])
    }

    pub fn brightness(self, value: f32) -> Self {
        self.add_effect("brightness", &[&value.to_string()])
    }

    pub fn contrast(self, value: f32) -> Self {
        self.add_effect("contrast", &[&value.to_string()])
    }

    pub fn watermark(
        self,
        text: &str,
        opacity: f32,
        size: u32,
        angle: i16,
        mode: u8,
        smoothness: u8,
    ) -> Self {
        let mode_str = if mode == 1 { "1" } else { "0" };
        self.add_effect(
            "watermark_visible",
            &[
                text,
                &opacity.to_string(),
                &size.to_string(),
                &angle.to_string(),
                mode_str,
                &smoothness.to_string(),
            ],
        )
    }
}
