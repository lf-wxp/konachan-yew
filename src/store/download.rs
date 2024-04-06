use bounce::Atom;
use serde::{Deserialize, Serialize};

use super::ImageState;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Default)]
pub(crate) struct Download {
  pub preview: String,
  pub url: String,
  pub percent: f32,
  pub status: ImageState,
}

#[derive(Atom, PartialEq, Clone, Debug, Default)]
pub(crate) struct Downloads(pub Vec<Download>);

impl Downloads {
  pub fn value(&self) -> &Vec<Download> {
    &self.0
  }
  pub fn push(&mut self, item: Download) -> Self {
    let mut value = self.value().clone();
    value.push(item);
    Downloads(value)
  }
}

impl From<Vec<Download>> for Downloads {
  fn from(value: Vec<Download>) -> Self {
    Downloads(value)
  }
}
