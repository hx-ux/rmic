use crate::parameter::Parameter;

/// Represents a G'MIC effect
#[derive(Debug, Clone)]
pub struct Effect {
    /// Short name of the effect (optional).
    pub name: Option<String>,
    /// The G'MIC command name.
    pub command: String,
    /// List of parameters and other elements.
    pub parameters: Vec<Parameter>,
    /// a raw string with all the commands in it
    raw: bool,
}

impl Effect {
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

    pub fn get_parameters(&self) -> Vec<&Parameter> {
        self.parameters.iter().collect()
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

        let params: String = self
            .get_parameters()
            .iter()
            .map(|p| p.default.clone())
            .collect::<Vec<_>>()
            .join(",");

        vec![self.command.clone(), params]
    }
}
