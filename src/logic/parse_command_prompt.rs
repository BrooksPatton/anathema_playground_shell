use std::str::Chars;

#[derive(Debug)]
pub struct ParsedCommandPrompt {
    pub command_name: Option<String>,
    pub arguments: Vec<String>,
}

impl ParsedCommandPrompt {
    pub fn new(command_prompt: String) -> Self {
        let mut command_prompt_iter = command_prompt.chars();
        let command_name = Self::extract_command_name(&mut command_prompt_iter);
        let arguments = Self::extract_arguments(&mut command_prompt_iter);

        Self {
            command_name,
            arguments,
        }
    }

    pub fn is_valid(&self) -> bool {
        true
    }

    fn extract_command_name(command_prompt_iter: &mut Chars<'_>) -> Option<String> {
        let mut command_name = String::new();

        loop {
            if let Some(current_char) = command_prompt_iter.next() {
                match current_char {
                    ' ' => break,
                    _ => command_name.push(current_char),
                }
            } else {
                break;
            }
        }

        if command_name.is_empty() {
            None
        } else {
            Some(command_name)
        }
    }

    fn extract_arguments(command_prompt_iter: &mut Chars<'_>) -> Vec<String> {
        command_prompt_iter
            .collect::<String>()
            .split_whitespace()
            .map(ToOwned::to_owned)
            .collect()
    }
}
