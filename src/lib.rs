use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
};
#[derive(Debug)]
pub enum GmicError {
    Io(io::Error),
    ExecutionFailed(String),
    InputNotFound,
    BinNotFound,
}

impl From<io::Error> for GmicError {
    fn from(err: io::Error) -> Self {
        GmicError::Io(err)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum GmicEffectType {
    Raw,
    Effect,
}

#[derive(Debug, Clone)]
pub struct GmicEffect {
    command: String,
    values: Vec<String>,
    prefix: Option<String>,
    effect_type: GmicEffectType,
}

impl GmicEffect {
    pub(crate) fn new(command: &str, params: &[&str], effect_type: GmicEffectType) -> Self {
        Self {
            command: command.to_string(),
            values: params.iter().map(|&s| s.to_string()).collect(),
            effect_type,
            prefix: None,
        }
    }
    
    // https://gmic.eu/tutorial/selections.html
    pub fn forargs(&self) -> Vec<String> {
        let mut args = Vec::<String>::new();

        if self.effect_type == GmicEffectType::Raw {
            for gg in &self.values {
                args.push(gg.clone());
            }
            return args;
        }

        let prefix = self.prefix.clone().unwrap_or_else(|| String::new());

        args.push(format!(
            "{}{}{}",
            prefix,
            self.command.clone(),
            String::new()
        ));

        //Multiple parameters must be provided as a single comma-separated string
        if !self.values.is_empty() {
            args.push(self.values.join(","));
        } else {
            // G'MIC expects a parameter even if empty; a single comma acts as a placeholder for default values.
            args.push(",".to_string());
        }
        args
    }
}

/// builder struct for G'MIC commands
pub struct Gmic {
    binary: String,
    effect_args: Vec<GmicEffect>,
    input_file: Option<PathBuf>,
    output_file: Option<PathBuf>,
}

impl Gmic {
    /// Assumes 'gmic' is in the system PATH.
    pub fn new() -> Self {
        Self {
            binary: "gmic".to_string(),
            effect_args: Vec::new(),
            input_file: None,
            output_file: None,
        }
    }

    /// FALLBACK  
    /// Specify a custom path to the gmic binary
    pub fn with_binary(path: &str) -> Self {
        Self {
            binary: path.to_string(),
            effect_args: Vec::new(),
            input_file: None,
            output_file: None,
        }
    }

    /// Select a file
    pub fn input<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.input_file = Some(path.as_ref().to_path_buf());
        self
    }

    /// Output the result to a file
    pub fn output<P: AsRef<Path>>(mut self, path: P) -> Self {
       self.output_file = Some(path.as_ref().to_path_buf());
       self
    }

    // --- Core Logic ---

    /// No Stack position will be set, these command applies to all images in the stack
    /// # Arguments
    /// * `op_name` - The G'MIC command name (e.g., "blur", "resize").
    ///               (The leading '-' is added automatically).
    /// * `params` - A slice of parameters/arguments for that command.
    ///                (Delimiter  ',' is added automatically).
    ///
    pub fn add_command(self, op_name: &str, params: &[&str]) -> Self {
        self.add_cmd(op_name, params, GmicEffectType::Effect).0
    }

    /// select your stack postion
    /// # Arguments
    /// * `op_name` - The G'MIC command name (e.g., "blur", "resize").
    ///               (The leading '-' is added automatically).
    /// * `params` - A slice of parameters/arguments for that command.
    ///                (Delimiter  ',' is added automatically).
    ///
    pub fn add_command_at(self, op_name: &str, params: &[&str], pos: Option<Vec<u16>>) -> Self {
        self.add_cmd(op_name, params, GmicEffectType::Effect).0
    }

    /// helper to handle the arg pushing logic
    fn add_cmd<'a>(mut self, op: &str, params: &[&str], e_type: GmicEffectType) -> (Self, ()) {
        self.effect_args.push(GmicEffect::new(op, params, e_type));
        (self, ())
    }

    /// Raw arguments for specific syntax control.
    pub fn add_raw_arg(mut self, arg: &str) -> Self {
        let parts: Vec<&str> = arg.split_whitespace().collect();
        if !parts.is_empty() {
        self.effect_args
            .push(GmicEffect::new("", &parts, GmicEffectType::Raw));
        }
        self
    }

    /// Executes the constructed G'MIC command.
    pub fn execute(&self) -> Result<(), GmicError> {
        let mut command = Command::new(&self.binary);

        if let Some(i) = &self.input_file {
            command.arg("-input");
            command.arg(i);
        }

        for effect in &self.effect_args {
            for arg in effect.forargs() {
                command.arg(arg);
            }
        }

        if let Some(i) = &self.output_file {
            command.arg("-output");
            command.arg(i);
        }

        println!("Executing: {:?} {:?}", self.binary, command.get_args());
        println!(
            "Command: {} {}",
            self.binary,
            command
                .get_args()
                .map(|arg| arg.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ")
        );
        let output = command.output()?;

        if output.status.success() {
            return Ok(());
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GmicError::ExecutionFailed(stderr.to_string()));
        }
    }

    // --- UTILS ---
    /// Generic file checker
    fn exist_file<P: AsRef<Path>>(path: P) -> (bool, PathBuf) {
        let path = path.as_ref();
        (path.exists(), path.to_path_buf())
    }

    /// Collection of common used params wrapped into methods for easier use.
    pub fn to_rgba(self) -> Self {
        self.add_raw_arg("to_rgba")
    }

    /// Force selected images to be in GRAY mode.
    pub fn to_gray(self) -> Self {
        self.add_raw_arg("to_gray")
    }

    /// Solarize selected images.
    pub fn solarize(self) -> Self {
        self.add_raw_arg("solarize")
    }

    /// Rotate images by degrees.
    pub fn rotate(self, degree: u16) -> Self {
        self.add_command("rotate", &[&degree.to_string()])
    }

    /// Blur selected images.
    pub fn blur(self, radius: f32) -> Self {
        self.add_command("blur", &[&radius.to_string()])
    }

    /// Resize selected images.
    pub fn resize(self, width: u32, height: u32) -> Self {
        self.add_command("resize", &[&width.to_string(), &height.to_string()])
    }

    /// Adjust brightness.
    pub fn brightness(self, value: f32) -> Self {
        self.add_command("brightness", &[&value.to_string()])
    }

    /// Adjust contrast.
    pub fn contrast(self, value: f32) -> Self {
        self.add_command("contrast", &[&value.to_string()])
    }

    /// Adds a gmic watermark with custom text.
    pub fn watermark(self, text: &str, opacity: f32, size: u32, angle: i16, mode: u8) -> Self {
        let mode_value = if mode == 1 { "1" } else { "0" };

        self.add_command(
            "watermark_visible",
            &[
                text,
                &opacity.to_string(),
                &size.to_string(),
                &angle.to_string(),
                mode_value, // Mode: Add
                "0",        // Smoothness
            ],
        )
    }
}
