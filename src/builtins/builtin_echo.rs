use crate::{builtins::Builtin, logic::run_command::CommandOutput};

pub fn builtin_echo(arguments: &[String]) -> CommandOutput {
    let standard_out = Some(arguments.join(" "));
    let standard_error = None;

    CommandOutput {
        command_name: Some(format!("{}", Builtin::Echo)),
        standard_out,
        standard_error,
    }
}
