use anathema::{
    component::{Component, KeyCode},
    state::{State, Value},
};

#[derive(Default, Debug, State)]
pub struct CommandPromptState {
    pub input: Value<String>,
    pub ps1: Value<String>,
    pub display_ps1: Value<bool>,
}

impl CommandPromptState {
    pub fn new() -> Self {
        let input = Value::new(String::new());
        let ps1 = Value::new(String::from("$ "));
        let display_ps1 = Value::new(true);

        Self {
            input,
            ps1,
            display_ps1,
        }
    }
}

pub struct CommandPrompt;

impl Component for CommandPrompt {
    type State = CommandPromptState;

    type Message = ();

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match key.code {
            KeyCode::Char(letter) => state.input.to_mut().push(letter),
            KeyCode::Backspace => {
                state.input.to_mut().pop();
            }
            KeyCode::Enter => {
                let mut input = state.input.to_mut();
                let display_ps1 = state.display_ps1.to_mut();
                let ps1 = state.ps1.to_ref();
                let command_prompt = format!(
                    "{}{}",
                    if display_ps1.as_bool().unwrap() {
                        ps1.as_str()
                    } else {
                        ""
                    },
                    input.as_str()
                );
                context
                    .components
                    .by_name("scrollback_buffer")
                    .send(command_prompt);
                input.clear();
            }
            _ => (),
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
}
