use bounce::Atom;
use std::fmt::{self, Display};

#[derive(Atom, PartialEq)]
pub struct Size(f64);

impl Size {
  pub fn new(num: f64) -> Self {
    Size(num)
  }
  pub fn value(&self) -> f64 {
    self.0
  }
}

impl Display for Size {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Default for Size {
  fn default() -> Self {
    Size(0.0)
  }
}
