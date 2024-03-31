use bounce::Atom;
use std::fmt::{self, Display};

#[derive(Atom, PartialEq)]
pub struct Page(u32);

impl Page {
  pub fn new(num: u32) -> Self {
    Page(num)
  }
  pub fn value(&self) -> u32 {
    self.0
  }
}

impl Display for Page {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Default for Page {
  fn default() -> Self {
    Page(5)
  }
}

#[derive(Atom, PartialEq)]
pub struct Total(u32);

impl Total {
  pub fn new(num: u32) -> Self {
    Total(num)
  }
  pub fn value(&self) -> u32 {
    self.0
  }
}

impl Display for Total {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Default for Total {
  fn default() -> Self {
    Total(100)
  }
}
