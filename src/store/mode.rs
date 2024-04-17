use bounce::Atom;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

use super::Invertible;

#[derive(Atom, PartialEq, Serialize, Deserialize, Default, Clone, Debug)]
pub(crate) enum Mode {
  #[default]
  Json,
  Xml,
}

impl Display for Mode {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let text = match self {
      Mode::Json => "json",
      Mode::Xml => "xml",
    };
    write!(f, "{}", text)
  }
}

impl Invertible for Mode {
  fn invert(&self) -> Self {
    match self {
      Mode::Json => Mode::Xml,
      Mode::Xml => Mode::Json,
    }
  }
}
