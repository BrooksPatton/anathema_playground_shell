use crate::components::shell::ShellState;

pub fn builtin_exit(context: &mut anathema::component::Context<'_, '_, ShellState>) {
    context.stop_runtime();
}
