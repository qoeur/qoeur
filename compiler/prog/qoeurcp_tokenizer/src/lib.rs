//! the tokenizer

// the tokenizer try to follow the html5ever parser implementation
// @see html5ever: https://github.com/servo/html5ever

// TODO: tmp
#![allow(dead_code)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(decl_macro)]
#![recursion_limit = "256"]

#[macro_use]
extern crate serde_derive;

mod ast;
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

pub use self::token::{Token, TokenPrinter, TokenQueue, TokenSink};
pub use self::tokenizer::{Tokenizer, TokenizerOpts};
pub use self::tree_builder::{TreeBuilder, TreeSink};

use self::ast::Stmt;

use std::borrow::Cow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use tendril::StrTendril;

#[derive(Clone, Debug)]
pub struct Handle(Rc<RefCell<Vec<Stmt>>>);

impl Deref for Handle {
  type Target = Rc<RefCell<Vec<Stmt>>>;
  fn deref(&self) -> &Rc<RefCell<Vec<Stmt>>> {
    &self.0
  }
}

pub trait ParseResult {
  type Sink: TreeSink + Default;
  fn get_result(sink: Self::Sink) -> Self;
}

#[derive(Debug)]
pub struct Tree {
  pub stmts: Handle,
}

impl Tree {
  pub fn new() -> Tree {
    Self {
      stmts: Handle(Rc::new(RefCell::new(vec![]))),
    }
  }
}

impl TreeSink for Tree {
  type Handle = Handle;
  fn get_stmts(&mut self) -> Self::Handle {
    self.stmts.clone()
  }

  fn parse_error(&mut self, _msg: Cow<'static, str>) {}
}

pub fn parse(file: &str) -> Tree {
  // TODO: opts must be use has argument i.e parse(file: &str, opts: TokenizerOpts)
  let opts = TokenizerOpts {
    profile: true,
    exact_errors: true,
    ..Default::default()
  };

  let buffer = StrTendril::from(file);
  let token_parser = Tree::new();
  let tree_builder = TreeBuilder::new(token_parser);
  let mut tokenizer = Tokenizer::new(tree_builder, opts);
  let _ = tokenizer.feed(buffer.try_reinterpret().unwrap());
  let _ = tokenizer.end();

  tokenizer.unwrap().unwrap()
}

pub fn tokenize(file: &str) -> TokenQueue {
  // TODO: opts must be use has argument i.e parse(file: &str, opts: TokenizerOpts)
  let opts = TokenizerOpts {
    profile: true,
    exact_errors: true,
    ..Default::default()
  };

  let buffer = StrTendril::from(file);
  let token_printer = TokenPrinter::new();
  let mut tokenizer = Tokenizer::new(token_printer, opts);
  let _ = tokenizer.feed(buffer.try_reinterpret().unwrap());
  let _ = tokenizer.end();

  tokenizer.token_queue
}
