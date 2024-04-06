use bounce::Atom;
use serde::{Deserialize, Serialize};
use bounce::{BounceStates, Selector};
use rand::{seq::SliceRandom, thread_rng};
use std::{fmt::{self, Display}, rc::Rc};

use crate::utils::{calc_waterfall, WaterfallParams};

use super::{Security, Size};

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum ImageState {
  Loaded,
  #[default]
  Pending,
  Error,
}

impl Display for ImageState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let text = match self {
      ImageState::Loaded => "loaded",
      ImageState::Pending => "pending",
      ImageState::Error => "error",
    };
    write!(f, "{}", text)
  }
}

#[derive(PartialEq, Clone, Deserialize, Debug, Default)]
pub(crate) struct Image {
  pub id: u32,
  pub sample_width: f64,
  pub sample_height: f64,
  pub sample: String,
  pub preview_width: f64,
  pub preview_height: f64,
  pub preview: String,
  pub url: String,
  pub width: f64,
  pub height: f64,
  pub security: bool,
  pub name: String,
  pub tags: Option<String>,
  pub style_h: Option<f64>,
  pub style_w: Option<f64>,
  pub style: Option<String>,
  pub full: Option<bool>,
}

#[derive(Atom, PartialEq, Clone, Debug, Default)]
pub(crate) struct Images(pub Vec<Image>);

impl Images {
  pub fn value(&self) -> &Vec<Image> {
    &self.0
  }
}

impl From<Vec<Image>> for Images {
  fn from(mut value: Vec<Image>) -> Self {
    let mut rng = thread_rng();
    value.shuffle(&mut rng);
    Images(value)
  }
}

#[derive(Atom, PartialEq, Clone, Debug, Default)]
pub(crate) struct FilterImages(Vec<Image>);

impl Selector for FilterImages {
  fn select(states: &BounceStates) -> Rc<Self> {
    let images = states.get_atom_value::<Images>();
    let security = states.get_atom_value::<Security>();
    let size = states.get_atom_value::<Size>();
    let items = calc_waterfall(WaterfallParams {
      security: *security.value(),
      width: *size.value(),
      max_width: 300.0,
      min_width: 200.0,
      images: images.value().to_vec(),
    });
    Rc::from(FilterImages(items))
  }
}

impl FilterImages {
  pub fn value(&self) -> &Vec<Image> {
    &self.0
  }
}
