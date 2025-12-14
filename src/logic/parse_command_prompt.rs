#[derive(Debug, Default)]
pub struct ParsedCommandPrompt {
    pub command_name: Option<String>,
}

impl ParsedCommandPrompt {
    pub fn new(command_prompt: &str) -> Self {
        let command_name = if command_prompt.is_empty() {
            None
        } else {
            Some(command_prompt.to_owned())
        };

        Self { command_name }
    }

    pub fn is_valid(&self) -> bool {
        true
    }
}
