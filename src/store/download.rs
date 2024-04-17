use bounce::Slice;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::Reducible;

use crate::model::DownloadProgress;

use super::ImageState;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Default)]
pub(crate) struct Download {
  pub preview: String,
  pub url: String,
  pub percent: f32,
  pub status: ImageState,
}

pub(crate) enum DownloadAction {
  UnShift(Download),
  Push(Download),
  Update(DownloadProgress),
}

#[derive(Slice, PartialEq, Clone, Debug, Default)]
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

impl Reducible for Downloads {
  type Action = DownloadAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      DownloadAction::Push(item) => {
        let mut value = self.value().clone();
        value.push(item);
        Downloads(value).into()
      }
      DownloadAction::UnShift(item) => {
        let mut value = self.value().clone();
        value.insert(0, item);
        Downloads(value).into()
      }
      DownloadAction::Update(download_progress) => {
        let DownloadProgress {
          url,
          percent,
          status,
        } = download_progress;
        let mut value = self.value().clone();
        if let Some(download) = value.iter_mut().find(|s| s.url == url) {
          download.percent = percent * 100.0;
          download.status = status;
        }
        Downloads(value).into()
      }
    }
  }
}
