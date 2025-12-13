use anathema::{
    component::Component,
    state::{List, State, Value},
};

#[derive(Debug, State, Default)]
pub struct ScrollbackBufferState {
    pub buffer: Value<List<String>>,
}

impl ScrollbackBufferState {
    pub fn new() -> Self {
        let buffer = Value::new(List::empty());

        Self { buffer }
    }
}

pub struct ScrollbackBuffer;

impl Component for ScrollbackBuffer {
    type State = ScrollbackBufferState;

    type Message = String;

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state.buffer.push_back(message);
    }

    fn accept_focus(&self) -> bool {
        false
    }
}
