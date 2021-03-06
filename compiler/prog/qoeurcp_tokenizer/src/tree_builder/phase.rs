pub use self::Phase::*;
pub use self::ProcessResult::*;

use crate::token::Token;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Phase {
  StartPhase,
  MainPhase,
  EndPhase,
}

pub enum ProcessResult {
  Done,
  Reprocess(Phase, Token),
}
