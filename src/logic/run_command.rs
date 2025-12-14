use std::fmt::Display;

use crate::{
    builtins::{
        Builtin, builtin_echo::builtin_echo, builtin_exit::builtin_exit,
        builtin_type::builtin_type, run_executable::run_executable,
    },
    components::shell::ShellState,
    logic::parse_command_prompt::ParsedCommandPrompt,
};

#[derive(Debug)]
pub struct CommandOutput {
    pub standard_out: Option<String>,
    pub standard_error: Option<RunCommandError>,
}

#[derive(Debug)]
pub enum RunCommandError {
    NotFound(String),
    MissingCommandName,
    UnknownError,
}

impl Display for RunCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::NotFound(command_name) => format!("{command_name}: command not found"),
            Self::MissingCommandName => format!("Error: a command name must be given"),
            Self::UnknownError => format!("Something went wrong"),
        };

        write!(f, "{message}")
    }
}

pub fn run_command(
    command: ParsedCommandPrompt,
    context: &mut anathema::component::Context<'_, '_, ShellState>,
) -> CommandOutput {
    let Some(command_name) = command.command_name.clone() else {
        if command.is_valid() {
            return CommandOutput {
                standard_out: None,
                standard_error: None,
            };
        }

        return CommandOutput {
            standard_out: None,
            standard_error: Some(RunCommandError::MissingCommandName),
        };
    };

    match Builtin::from(&command_name) {
        Builtin::Exit => {
            builtin_exit(context);
            unreachable!();
        }
        Builtin::Echo => builtin_echo(&command.arguments),
        Builtin::Type => builtin_type(&command.arguments),
        Builtin::Notfound => run_executable(&command_name, &command.arguments),
    }
}
