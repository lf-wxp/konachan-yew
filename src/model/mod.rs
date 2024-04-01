use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

use crate::store::Image;

pub type Error = Box<dyn std::error::Error>;
pub type UResult<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImageData {
  pub images: Vec<Image>,
  pub count: u32,
}
#[derive(PartialEq, Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImageRes {
  pub code: i32,
  pub data: ImageData,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Action {
  GetPost,
  DownloadItem,
  CloseSplashscreen,
}

impl Display for Action {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let text = match self {
      Action::GetPost => "get_post",
      Action::DownloadItem => "download_item",
      Action::CloseSplashscreen => "close_splashscreen",
    };
    write!(f, "{}", text)
  }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FetchMode {
  Xml,
  Json,
}

impl Display for FetchMode {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let text = match self {
      FetchMode::Json => "json",
      FetchMode::Xml => "xml",
    };
    write!(f, "{}", text)
  }
}
#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct FetchParams {
  page: u32,
  tags: Option<String>,
  refresh: Option<bool>,
  mode: Option<FetchMode>,
}

impl FetchParams {
  pub fn new(
    page: u32,
    tags: Option<String>,
    refresh: Option<bool>,
    mode: Option<FetchMode>,
  ) -> Self {
    FetchParams {
      page,
      tags,
      refresh,
      mode,
    }
  }
  pub fn param(&self) -> Vec<(&'static str, String)> {
    let FetchParams {
      page,
      tags,
      refresh,
      mode,
    } = self.clone();
    let mut params: Vec<(&'static str, String)> = Vec::new();
    params.push(("page", page.to_string()));
    if let Some(tag) = tags {
      params.push(("tags", tag));
    }
    if let Some(refr) = refresh {
      params.push(("refresh", refr.to_string()));
    }
    if let Some(m) = mode {
      params.push(("mode", m.to_string()));
    }
    params
  }
}
