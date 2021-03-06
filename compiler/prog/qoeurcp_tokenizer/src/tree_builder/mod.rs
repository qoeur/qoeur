mod interface;
mod phase;

pub use self::interface::{TreeBuilderStep, TreeSink};
pub use self::phase::{Phase, ProcessResult};

use crate::token::{Token, TokenSink};

use std::collections::VecDeque;

pub struct TreeBuilder<Handle, Sink> {
  current_elmt: Option<Handle>,
  handle: Handle,
  next_tokenizer_state: Option<Phase>,
  open_nodes: Vec<Handle>,
  phase: Phase,
  sink: Sink,
}

impl<Handle, Sink> TokenSink for TreeBuilder<Handle, Sink>
where
  Handle: Clone,
  Sink: TreeSink<Handle=Handle>,
{
  fn end(&mut self) {}
  fn print(&self, _level: usize) {}

  fn process_token(&mut self, token: Token) {
    self.process_to_completion(token);
  }
}

impl<Handle, Sink> TreeBuilder<Handle, Sink>
where
  Handle: Clone,
  Sink: TreeSink<Handle=Handle>,
{
  pub fn new(mut sink: Sink) -> TreeBuilder<Handle, Sink> {
    let handle = sink.get_program();

    Self {
      current_elmt: None,
      handle: handle,
      next_tokenizer_state: None,
      open_nodes: vec![],
      phase: Phase::StartPhase,
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
        ProcessResult::Done => {
          token = mac::unwrap_or_return!(more_tokens.pop_front(), ());
        }
        ProcessResult::Reprocess(m, t) => {
          self.phase = m;
          token = t;
        }
      }
    }
  }
}

impl<Handle, Sink> TreeBuilderStep
    for super::TreeBuilder<Handle, Sink>
    where Handle: Clone,
          Sink: TreeSink<Handle=Handle>,
{
  fn step(&mut self, _mode: Phase, _token: Token) -> ProcessResult {
    ProcessResult::Done
  }
}