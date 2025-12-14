use std::fmt::Display;

use crate::logic::parse_command_prompt::ParsedCommandPrompt;

#[derive(Debug)]
pub struct CommandOutput {
    pub standard_out: Option<String>,
    pub standard_error: Option<RunCommandError>,
    pub exit_code: u32,
}

#[derive(Debug)]
pub enum RunCommandError {
    NotFound(String),
    MissingCommandName,
}

impl Display for RunCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::NotFound(command_name) => format!("{command_name}: command not found"),
            Self::MissingCommandName => format!("Error: a command name must be given"),
        };

        write!(f, "{message}")
    }
}

pub fn run_command(command: ParsedCommandPrompt) -> CommandOutput {
    let Some(command_name) = command.command_name.clone() else {
        if command.is_valid() {
            return CommandOutput {
                standard_out: None,
                standard_error: None,
                exit_code: 0,
            };
        }

        return CommandOutput {
            standard_out: None,
            standard_error: Some(RunCommandError::MissingCommandName),
            exit_code: 1,
        };
    };

    CommandOutput {
        standard_out: None,
        standard_error: Some(RunCommandError::NotFound(command_name)),
        exit_code: 1,
    }
}
