//! # RMIC: Rust G'MIC Interface
//!
//! This library provides a builder pattern for constructing and executing G'MIC commands.
//! It allows chaining image processing effects, setting input/output files, and executing via the CLI.

mod filter;
mod parameter;

use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{filter::Filter, parameter::Parameter};

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
    GmicNotFound,
    /// JSON could not be parsed.
    JsonParseError,
    /// JSON could not be parsed.
    EmptyEffectChain,
}

impl From<io::Error> for GmicError {
    fn from(err: io::Error) -> Self {
        GmicError::Io(err)
    }
}

pub struct Gmic {
    pub binary: String,
    pub filters: Vec<Filter>,
    pub input_file: Option<PathBuf>,
    pub output_file: Option<PathBuf>,
}

impl Default for Gmic {
    fn default() -> Self {
        Self {
            binary: "gmic".to_string(),
            filters: Vec::new(),
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

        self.filters
            .push(Filter::new(command.to_string(), parameters));
        self
    }

    /// Adds raw arguments.
    pub fn add_raw_effect(mut self, arg: &str) -> Self {
        if !arg.is_empty() {
            self.filters.push(Filter::new_raw(arg.to_string()));
        }
        self
    }

    pub fn add_json_effect(mut self, json: &str) -> Self {
        match Filter::from_json(json) {
            Ok(effect) => {
                self.filters.push(effect);
                self
            }
            Err(_) => {
                log::warn!("Failed to parse JSON effect: {}", json);
                self
            }
        }
    }

    pub fn add_build_effect(mut self, effect: Filter) -> Self {
        self.filters.push(effect);
        self
    }

    pub fn randomize(mut self) -> Self {
        for eff in self.filters.iter_mut() {
            eff.randomize();
        }
        self
    }

    fn generate_command(&self) -> Command {
        let mut command = Command::new(&self.binary);

        if let Some(ref input) = self.input_file {
            command.arg("-input").arg(input);
        }

        for effect in &self.filters {
            command.args(effect.forargs());
        }

        if let Some(ref output) = self.output_file {
            command.arg("-output").arg(output);
        }

        command
    }
    /// Executes the built command.
    pub fn execute(&self) -> Result<(), GmicError> {
        if self.filters.iter().count() == 0 {
            return Err(GmicError::EmptyEffectChain);
        }

        if let Some(ref input) = self.input_file
            && !input.exists()
        {
            return Err(GmicError::InputNotFound);
        }

        let output = self.generate_command().output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(GmicError::ExecutionFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
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

impl Gmic {
    pub fn to_rgba(self) -> Self {
        self.add_raw_effect("to_rgba")
    }

    pub fn to_gray(self) -> Self {
        self.add_raw_effect("to_gray")
    }

    pub fn solarize(self) -> Self {
        self.add_raw_effect("solarize")
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

    pub fn display(self) -> Self {
        self.add_effect("-display", &[])
    }
}
