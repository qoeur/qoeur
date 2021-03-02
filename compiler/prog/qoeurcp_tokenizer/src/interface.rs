use super::token::Token;

use std::fmt;

pub trait TokenSink {
  fn end(&mut self);
  fn print(&self, level: usize);
  fn process_token(&mut self, token: Token);
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum TokenKind {
  EOF,
}

impl fmt::Display for TokenKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl TokenKind {
  pub fn text(&self) -> String {
    match *self {
      Self::EOF => format!("EOF"),
    }
  }
}
