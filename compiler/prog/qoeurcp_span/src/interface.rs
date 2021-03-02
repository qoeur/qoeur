use std::fmt;

pub type RawIndex = u32;

pub trait Pos {
  fn from_usize(n: usize) -> Self;
  fn to_usize(&self) -> usize;
  fn from_u32(n: u32) -> Self;
  fn to_u32(&self) -> u32;
  fn zero() -> Self;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[derive(PartialOrd, Ord, Serialize, Deserialize)]
pub struct PosIndex(pub RawIndex);

impl Pos for PosIndex {
  fn from_usize(n: usize) -> Self {
    PosIndex(n as u32)
  }

  fn to_usize(&self) -> usize {
    self.0 as usize
  }

  fn from_u32(n: u32) -> Self {
    PosIndex(n)
  }

  fn to_u32(&self) -> u32 {
    self.0
  }

  fn zero() -> Self {
    Self(0)
  }
}

impl fmt::Display for PosIndex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl PosIndex {
  pub fn number(&self) -> Self {
    PosIndex(self.0 + 1)
  }

  pub fn text(&self) -> String {
    format!("PosIndex({})", self.0)
  }
}
