use bounce::Atom;
use serde::Deserialize;
use bounce::{BounceStates, Selector};
use std::rc::Rc;

use crate::utils::{calc_waterfall, WaterfallParams};

use super::{Security, Size};

#[derive(PartialEq, Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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

#[derive(Atom, PartialEq, Clone, Debug)]
pub(crate) struct Images(Vec<Image>);

impl Images {
  pub fn value(&self) -> &Vec<Image> {
    &self.0
  }
}

impl Default for Images {
  fn default() -> Self {
    return Images(vec![]);
  }
}

impl From<Vec<Image>> for Images {
  fn from(value: Vec<Image>) -> Self {
    Images(value)
  }
}

#[derive(Atom, PartialEq, Clone, Debug)]
pub(crate) struct FilterImages(Vec<Image>);

impl Selector for FilterImages {
  fn select(states: &BounceStates) -> Rc<Self> {
    let images = states.get_atom_value::<Images>();
    let security = states.get_atom_value::<Security>();
    let size = states.get_atom_value::<Size>();
    let items = calc_waterfall(WaterfallParams {
      security: security.value(),
      width: size.value(),
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

impl Default for FilterImages {
  fn default() -> Self {
    return FilterImages(vec![]);
  }
}