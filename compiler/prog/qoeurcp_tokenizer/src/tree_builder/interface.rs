use crate::ast::Stmt;

use std::borrow::Cow;

pub trait TreeBuilderPrinter {
  fn print(&mut self, stmt: Box<Stmt>);
}

pub trait TreeSink {
  type Handle: Clone;
  fn get_stmts(&mut self) -> Self::Handle;
  fn parse_error(&mut self, msg: Cow<'static, str>);
}

pub trait TreePrinter {
  fn print(&self, level: usize);
}
