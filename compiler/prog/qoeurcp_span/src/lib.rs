#![feature(box_patterns)]
#![feature(box_syntax)]
#![recursion_limit = "256"]

#[macro_use]
extern crate serde_derive;

mod interface;
mod loc;
mod span;

#[cfg(test)]
mod test;

pub use self::interface::{Pos, PosIndex};
pub use self::loc::Loc;
pub use self::span::Span;
