use super::{LitKind, Token, TokenKind, TokenSink};

pub struct TokenPrinter {
  tokens: Vec<Token>,
}

impl TokenPrinter {
  pub fn new() -> TokenPrinter {
    Self { tokens: vec![] }
  }
}

impl TokenSink for TokenPrinter {
  fn end(&mut self) {}

  fn print(&self, _level: usize) {}

  fn process_token(&mut self, token: Token) {
    println!("token_printer: {}", token);
    match token.kind {
      _ => {}
    }
  }
}
