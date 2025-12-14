use anathema::{
    component::{Component, KeyCode},
    state::{State, Value},
};

#[derive(Default, Debug, State)]
pub struct CommandPromptState {
    pub input: Value<String>,
}

impl CommandPromptState {
    pub fn new() -> Self {
        let input = Value::new(String::new());

        Self { input }
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
                let command_prompt = format!("{}", input.as_str());

                context.publish("command_entered", command_prompt);
                input.clear();
            }
            _ => (),
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
}
