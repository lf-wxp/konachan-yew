use bounce::Atom;
use serde::Deserialize;

#[derive(PartialEq, Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Image {
  id: u32,
  sample_width: u32,
  sample_height: u32,
  sample: String,
  preview_width: u32,
  preview_height: u32,
  preview: String,
  url: String,
  width: u32,
  height: u32,
  security: bool,
  name: String,
  tags: Option<String>,
}

#[derive(Atom, PartialEq, Clone, Debug)]
pub(crate) struct Images(Vec<Image>);

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
