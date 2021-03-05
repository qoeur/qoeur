pub use super::token::interface::TreeSink;

use crate::state::State;

use std::collections::VecDeque;

pub struct TreeBuilder<Handle, Sink> {
  sink: Sink,
  handle: Handle,
  next_tokenizer_state: Option<State>,
  current_elmt: Option<Handle>,
  phase: State,
}

impl<Handle, Sink> TreeBuilder<Handle, Sink>
where
  Handle: Clone,
  Sink: TreeSink<Handle=Handle>,
{
  pub fn new(mut sink: Sink) -> TreeBuilder<Handle, Sink> {
    let handle = sink.get_document();

    Self {
      curr_elem: None,
      handle: handle,
      next_tokenizer_state: None,
      open_elems: vec!(),
      phase: State::StartLine,
      sink: sink,
    }
  }

  pub fn unwrap(self) -> Sink {
    self.sink
  }

  pub fn sink<'a>(&'a self) -> &'a Sink {
    &self.sink
  }

  pub fn sink_mut<'a>(&'a mut self) -> &'a mut Sink {
    &mut self.sink
  }

  fn process_to_completion(&mut self, mut token: Token) {
    let mut more_tokens = VecDeque::new();

    loop {
      let phase = self.phase;

      match self.step(phase, token) {
        State::DONE => {
          token = unwrap_or_return!(more_tokens.pop_front(), ());
        }
        XReprocess(m, t) => {
          self.phase = m;
          token = t;
        }
      }
    }
  }
}
