use bounce::Atom;
use std::fmt::{self, Display};

#[derive(Atom, PartialEq)]
pub struct Security(bool);

impl Security {
  pub fn new(val: bool) -> Self {
    Security(val)
  }
  pub fn value(&self) -> bool {
    self.0
  }
}

impl Display for Security {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Default for Security {
  fn default() -> Self {
    Security(true)
  }
}
