use crate::{
    builtins::Builtin,
    logic::{
        find_executibles_in_path::find_first_executable_in_path_by_name,
        run_command::{CommandOutput, RunCommandError},
    },
};

pub fn builtin_type(arguments: &[String]) -> CommandOutput {
    let argument = arguments.first().cloned().unwrap_or_default();
    let builtin = Builtin::from(&argument);

    if !matches!(builtin, Builtin::Notfound) {
        return CommandOutput {
            standard_out: Some(format!("{builtin} is a shell builtin")),
            standard_error: None,
        };
    }

    if let Some(executable) = find_first_executable_in_path_by_name(&argument) {
        let path = executable.path();

        return CommandOutput {
            standard_out: Some(format!("{argument} is {}", path.display())),
            standard_error: None,
        };
    }

    CommandOutput {
        standard_out: None,
        standard_error: Some(RunCommandError::NotFound(argument)),
    }
}
