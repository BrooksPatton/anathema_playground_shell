use crate::{
    builtins::Builtin,
    logic::run_command::{CommandOutput, RunCommandError},
};

pub fn builtin_type(command_name: &str, arguments: &[String]) -> CommandOutput {
    let builtin = Builtin::from(&arguments.first().cloned().unwrap_or_default());
    let standard_error = if matches!(builtin, Builtin::Notfound) {
        Some(RunCommandError::NotFound(format!(
            "{command_name}: command not found"
        )))
    } else {
        None
    };
    let standard_out = Some(format!("{builtin} is a shell builtin"));

    CommandOutput {
        standard_out,
        standard_error,
    }
}
