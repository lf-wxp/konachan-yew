use crate::create_store;

use super::Invertible;

create_store!(Security, bool, true);

impl Invertible for Security {
  fn invert(&self) -> Self {
    Self(!self.0)
  }
}
