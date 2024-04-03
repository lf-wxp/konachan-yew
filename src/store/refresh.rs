use crate::create_store;

use super::Invertible;

create_store!(Refresh, bool, true);

impl Invertible for Refresh {
  fn invert(&self) -> Self {
    Self(!self.0)
  }
}
