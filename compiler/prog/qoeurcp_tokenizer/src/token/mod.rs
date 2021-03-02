pub use super::interface::TokenKind;

use qoeurcp_span::Span;

use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Token {
  #[allow(dead_code)]
  pub fn new(kind: TokenKind, span: Span) -> Token {
    Self { kind, span }
  }

  pub fn text(&self) -> String {
    format!("{}", self.kind.text())
  }
}
