#![allow(dead_code)]

pub use self::BinOpKind::*;
pub use self::IntBase::*;
pub use self::LitKind::*;
pub use self::TokenKind::*;
pub use self::UnOpKind::*;

use super::token::Token;

use std::fmt;

macro symbols {
  { $type:tt { $($kind:ident: $value:expr,)* } } => {
    impl std::fmt::Display for $type {
      fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
          $($kind => write!(f, "{}", $value),)*
        }
      }
    }
  },
  { $type:tt { $($kind:ident,)* } } => {
    impl std::fmt::Display for $type {
      fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
          $($kind(ref value) => write!(f, "{}", *value),)*
        }
      }
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinOpKind {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Lt,
  Gt,
  Le,
  Ge,
  Ne,
  Eq,
  EqEq,
  And,
  AndAnd,
  Or,
  OrOr,
  Dot,
  DotDot,
}

symbols! {
  BinOpKind {
    Add: "+",
    Sub: "-",
    Mul: "*",
    Div: "/",
    Mod: "%",
    Lt: "<",
    Gt: ">",
    Le: "<=",
    Ge: ">=",
    Ne: "!=",
    Eq: "=",
    EqEq: "==",
    And: "&",
    AndAnd: "&&",
    Or: "|",
    OrOr: "||",
    Dot: ".",
    DotDot: "..",
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntBase {
  Bin,
  Dec,
  Hex,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LitKind {
  Real(String),
  Int(String),
  Str(String),
  Char(char),
}

symbols! {
  LitKind {
    Real,
    Int,
    Str,
    Char, 
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
  EOF,
  EOL,
  Ident(String),
  Lit(LitKind),
  BinOp(BinOpKind),
  UnOp(UnOpKind),
  AssignOp(BinOpKind),
  Indent(usize),
  OpenBrace,
  CloseBrace,
  OpenBracket,
  CloseBracket,
  OpenParen,
  CloseParen,
  Arrow,
  ArrowFat,
  At,
  Attr,
  BackSlash,
  Colon,
  ColonColon,
  Comma,
  Dollar,
  DollarDotDot,
  QuestionMark,
  Shebang,
  Semicolon,
  As,
  Async,
  Await,
  Break,
  Capsule,
  Continue,
  Else,
  Enum,
  Exp,
  Ext,
  False,
  For,
  Fun,
  If,
  Load,
  Loop,
  Match,
  Module,
  Mut,
  Pub,
  Ref,
  Ret,
  SelfLower,
  SelfUpper,
  Set,
  Static,
  Struct,
  Super,
  True,
  Type,
  Typeof,
  Underscore,
  Unsafe,
  Use,
  Val,
  Void,
  While,
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
      Self::EOL => format!("EOL"),
      Self::OpenBrace => format!("{{"),
      Self::CloseBrace => format!("}}"),
      Self::OpenBracket => format!("["),
      Self::CloseBracket => format!("]"),
      Self::OpenParen => format!("("),
      Self::CloseParen => format!(")"),
      Self::Arrow => format!("->"),
      Self::ArrowFat => format!("=>"),
      Self::At => format!("@"),
      Self::Attr => format!("|>"),
      Self::BackSlash => format!("\\"),
      Self::Colon => format!(":"),
      Self::ColonColon => format!("::"),
      Self::Comma => format!(","),
      Self::Dollar => format!("$"),
      Self::DollarDotDot => format!("$.."),
      Self::QuestionMark => format!("?"),
      Self::Shebang => format!("#!"),
      Self::Semicolon => format!(";"),
      Self::As => format!("as"),
      Self::Async => format!("async"),
      Self::Await => format!("await"),
      Self::Break => format!("break"),
      Self::Capsule => format!("capsule"),
      Self::Continue => format!("continue"),
      Self::Else => format!("else"),
      Self::Enum => format!("enum"),
      Self::Exp => format!("exp"),
      Self::Ext => format!("ext"),
      Self::False => format!("false"),
      Self::For => format!("for"),
      Self::Fun => format!("fun"),
      Self::If => format!("if"),
      Self::Load => format!("load"),
      Self::Loop => format!("loop"),
      Self::Match => format!("match"),
      Self::Module => format!("mod"),
      Self::Mut => format!("mut"),
      Self::Pub => format!("pub"),
      Self::Ref => format!("ref"),
      Self::Ret => format!("ret"),
      Self::SelfLower => format!("self"),
      Self::SelfUpper => format!("Self"),
      Self::Set => format!("set"),
      Self::Static => format!("static"),
      Self::Struct => format!("struct"),
      Self::Super => format!("super"),
      Self::True => format!("true"),
      Self::Type => format!("type"),
      Self::Typeof => format!("typeof"),
      Self::Underscore => format!("_"),
      Self::Unsafe => format!("unsafe"),
      Self::Use => format!("use"),
      Self::Val => format!("val"),
      Self::Void => format!("void"),
      Self::While => format!("while"),
      Self::AssignOp(ref kind) => format!("<assign: `{}`>", kind),
      Self::BinOp(ref kind) => format!("<binop: `{}`>", kind),
      Self::Ident(ref ident) => format!("<ident: `{}`>", ident),
      Self::Indent(ref indent) => format!("<indent: `{}`>", indent),
      Self::Lit(ref lit) => format!("<lit: `{}`>", lit),
      Self::UnOp(ref unop) => format!("<unop: `{}`>", unop),
    }
  }
}

pub trait TokenSink {
  fn end(&mut self);
  fn print(&self, level: usize);
  fn process_token(&mut self, token: Token);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnOpKind {
  Not,
  Neg,
}

symbols! {
  UnOpKind {
    Not: "!",
    Neg: "-",
  }
}
