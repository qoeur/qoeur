//! the tokenizer

// the tokenizer try to follow the html5ever parser implementation
// @see html5ever: https://github.com/servo/html5ever

// tmp
#![allow(dead_code)]
// tmpend

#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(decl_macro)]
#![recursion_limit = "256"]

mod buffer_queue;

#[macro_use]
mod macros;

mod state;

#[cfg(test)]
mod test;

mod token;
mod tokenizer;
mod tree_builder;
mod util;

pub use self::tree_builder::TreeBuilder;
pub use self::token::{TokenPrinter, TokenQueue, TokenSink};
pub use self::tokenizer::{Tokenizer, TokenizerOpts};

use tendril::StrTendril;

pub fn tokenfile_program_to<Sink, It>(
  sink: Sink,
  input: It,
  opts: TokenizerOpts,
) -> Sink
where
  Sink: TokenSink,
  It: IntoIterator<Item = tendril::StrTendril>,
{
  let mut tokenizer = Tokenizer::new(sink, opts);

  input.into_iter().for_each(|s| tokenizer.feed(s));
  tokenizer.end();
  tokenizer.unwrap()
}

pub fn tokenfile(pathname: &str) -> TokenQueue {
  let path = std::path::Path::new(pathname);

  let f = match std::fs::read_to_string(&path) {
    Err(_) => None,
    Ok(file) => Some(file),
  };

  match f {
    None => TokenQueue::new(),
    Some(file) => {
      println!("\n{}", file);

      let opts = TokenizerOpts {
        profile: true,
        exact_errors: true,
        ..Default::default()
      };

      let buffer = StrTendril::from(file);
      let sink = TokenPrinter::new();
      let mut tokenizer = Tokenizer::new(sink, opts);
      let _ = tokenizer.feed(buffer.try_reinterpret().unwrap());
      let _ = tokenizer.end();

      tokenizer.token_queue
    }
  }
}
