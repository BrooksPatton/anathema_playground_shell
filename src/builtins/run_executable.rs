use crate::logic::{
    find_executibles_in_path::find_first_executable_in_path_by_name,
    run_command::{CommandOutput, RunCommandError},
};
use std::process::{self};

pub fn run_executable(command_name: &str, arguments: &[String]) -> CommandOutput {
    if find_first_executable_in_path_by_name(command_name).is_none() {
        return CommandOutput {
            command_name: Some(command_name.to_owned()),
            standard_out: None,
            standard_error: Some(RunCommandError::NotFound(command_name.to_owned())),
        };
    }

    let mut command = process::Command::new(command_name);

    command.args(arguments);

    let Ok(output) = command.output() else {
        return CommandOutput {
            command_name: Some(command_name.to_owned()),
            standard_out: None,
            standard_error: Some(RunCommandError::UnknownError),
        };
    };
    let standard_out = String::from_utf8(output.stdout)
        .ok()
        .map(|output| output.trim().to_owned());

    CommandOutput {
        command_name: Some(command_name.to_owned()),
        standard_out,
        standard_error: None,
    }
}
