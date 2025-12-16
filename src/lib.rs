use crate::components::command_prompt::{CommandPrompt, CommandPromptState};
use crate::components::scrollback_buffer::{ScrollbackBuffer, ScrollbackBufferState};
use crate::components::shell::{Shell, ShellState};
pub use crate::logic::run_command::CommandOutput as BBCommandOutput;
use anathema::backend::Backend;
use anathema::backend::tui::TuiBackend;
use anathema::runtime::{Builder, Runtime};
use anathema::templates::{Document, ToSourceKind};
use std::fs::read_to_string;
use std::path::Path;

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

pub fn register<'a>(builder: &mut Builder<()>, templates_path: &'static str) {
    let path = Path::new(templates_path);
    builder
        .component(
            "bb_command_prompt",
            path.join("command_prompt.aml"),
            CommandPrompt,
            CommandPromptState::new(),
        )
        .unwrap();
    builder
        .prototype(
            "bb_scrollback_buffer",
            path.join("scrollback_buffer.aml"),
            || ScrollbackBuffer,
            || ScrollbackBufferState::new(),
        )
        .unwrap();
    builder
        .prototype(
            "bb_shell",
            path.join("shell.aml"),
            || Shell,
            || ShellState::new(),
        )
        .unwrap();
}
