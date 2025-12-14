use anathema::{
    component::Component,
    state::{State, Value},
};

use crate::logic::{parse_command_prompt::ParsedCommandPrompt, run_command::run_command};

#[derive(State, Default, Debug)]
pub struct ShellState {
    pub ps1: Value<String>,
}

impl ShellState {
    pub fn new() -> Self {
        let ps1 = Value::new(String::from("$ "));

        Self { ps1 }
    }
}

pub struct Shell;

impl Component for Shell {
    type State = ShellState;

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "command_entered" => {
                let command_prompt = event.data::<String>().clone();
                let ps1 = state.ps1.to_ref();
                let parsed_command_prompt = ParsedCommandPrompt::new(&command_prompt);
                let run_result = run_command(parsed_command_prompt);

                if let Some(success_output) = run_result.standard_out {
                    context
                        .components
                        .by_name("scrollback_buffer")
                        .send(success_output);
                }

                if let Some(error_output) = run_result.standard_error {
                    context
                        .components
                        .by_name("scrollback_buffer")
                        .send(error_output.to_string());
                }

                context
                    .components
                    .by_name("scrollback_buffer")
                    .send(format!("{}{command_prompt}", ps1.as_str()));
            }
            _ => (),
        }
    }
}
