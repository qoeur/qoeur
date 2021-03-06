use super::phase::{Phase, ProcessResult};

use crate::token::Token;

use std::borrow::Cow;

pub trait TreeBuilderStep {
  fn step(&mut self, mode: Phase, token: Token) -> ProcessResult;
}

pub trait TreeSink {
  type Handle: Clone;

  fn parse_error(&mut self, msg: Cow<'static, str>);
  fn get_program(&mut self) -> Self::Handle;
}
