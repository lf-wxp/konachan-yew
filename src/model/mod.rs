use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

use crate::store::{Image, ImageState, Mode};

pub(crate) type Error = Box<dyn std::error::Error>;
pub(crate) type UResult<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImageData {
  pub images: Vec<Image>,
  pub count: usize,
}
#[derive(PartialEq, Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImageRes {
  pub code: i32,
  pub data: ImageData,
  pub msg: Option<String>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Action {
  GetPost,
  DownloadImage,
  CloseSplashscreen,
}

impl Display for Action {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let text = match self {
      Action::GetPost => "get_post",
      Action::DownloadImage => "download_image",
      Action::CloseSplashscreen => "close_splashscreen",
    };
    write!(f, "{}", text)
  }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub(crate) struct FetchParams {
  page: u32,
  tags: String,
  mode: Mode,
  limit: u32,
}

impl FetchParams {
  pub fn new(page: u32, tags: String, mode: Mode) -> Self {
    FetchParams {
      page,
      tags,
      mode,
      limit: 20,
    }
  }
  pub fn param(&self) -> Vec<(&'static str, String)> {
    let FetchParams {
      page,
      tags,
      mode,
      limit,
    } = self.clone();
    vec![
      ("page", page.to_string()),
      ("tags", tags),
      ("mode", mode.to_string()),
      ("limit", limit.to_string()),
    ]
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Empty;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DownloadParam {
  pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DownloadProgress {
  pub url: String,
  pub percent: f32,
  pub status: ImageState,
}
