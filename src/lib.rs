use anathema::backend::Backend;
use anathema::backend::tui::TuiBackend;
use anathema::runtime::Runtime;
use anathema::templates::{Document, ToSourceKind};
use std::fs::read_to_string;

use crate::components::command_prompt::{CommandPrompt, CommandPromptState};
use crate::components::scrollback_buffer::{ScrollbackBuffer, ScrollbackBufferState};
use crate::components::shell::{Shell, ShellState};

mod builtins;
mod components;
mod logic;

pub fn run() {
    let template = read_to_string("templates/index.aml").unwrap();
    let doc = Document::new("@index");
    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .enable_mouse()
        .finish()
        .unwrap();

    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);

    builder.template("index", template.to_template()).unwrap();

    builder
        .component(
            "command_prompt",
            "templates/command_prompt.aml",
            CommandPrompt,
            CommandPromptState::new(),
        )
        .unwrap();
    builder
        .prototype(
            "scrollback_buffer",
            "templates/scrollback_buffer.aml",
            || ScrollbackBuffer,
            || ScrollbackBufferState::new(),
        )
        .unwrap();
    builder
        .prototype(
            "shell",
            "templates/shell.aml",
            || Shell,
            || ShellState::new(),
        )
        .unwrap();

    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .unwrap();
}
