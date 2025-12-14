use crate::logic::run_command::CommandOutput;

pub fn builtin_echo(arguments: &[String]) -> CommandOutput {
    let standard_out = Some(arguments.join(" "));
    let standard_error = None;

    CommandOutput {
        standard_out,
        standard_error,
    }
}
