use super::Slice;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

use crate::model::DownloadProgress;

use super::ImageState;

/// A single download item with progress tracking.
#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Default)]
pub struct Download {
  pub preview: String,
  pub url: String,
  pub percent: f32,
  pub status: ImageState,
}

/// Actions that can be dispatched to the Downloads reducer.
#[allow(dead_code)]
pub enum DownloadAction {
  /// Add a download to the end of the list
  Push(Download),
  /// Add a download to the beginning of the list
  UnShift(Download),
  /// Update progress for an existing download by URL
  Update(DownloadProgress),
}

/// Collection of download items managed as a reducer slice.
#[derive(PartialEq, Clone, Debug, Default)]
pub struct Downloads(pub Vec<Download>);

impl Slice for Downloads {
  type Action = DownloadAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      DownloadAction::Push(item) => {
        let mut value = self.0.clone();
        value.push(item);
        Rc::new(Downloads(value))
      }
      DownloadAction::UnShift(item) => {
        let mut value = Vec::with_capacity(self.0.len() + 1);
        value.push(item);
        value.extend_from_slice(&self.0);
        Rc::new(Downloads(value))
      }
      DownloadAction::Update(progress) => {
        let mut value = self.0.clone();
        if let Some(download) = value.iter_mut().find(|s| s.url == progress.url) {
          download.percent = progress.percent * 100.0;
          download.status = progress.status;
        }
        Rc::new(Downloads(value))
      }
    }
  }
}

impl Downloads {
  /// Get a reference to the inner download list.
  pub fn value(&self) -> &Vec<Download> {
    &self.0
  }
}

impl From<Vec<Download>> for Downloads {
  fn from(value: Vec<Download>) -> Self {
    Downloads(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_download(url: &str) -> Download {
    Download {
      preview: String::new(),
      url: url.to_string(),
      percent: 0.0,
      status: ImageState::Pending,
    }
  }

  #[test]
  fn test_default_empty() {
    let d = Downloads::default();
    assert!(d.value().is_empty());
  }

  #[test]
  fn test_push_appends() {
    let d = Rc::new(Downloads::default());
    let d = d.reduce(DownloadAction::Push(make_download("a")));
    let d = d.reduce(DownloadAction::Push(make_download("b")));
    assert_eq!(d.value().len(), 2);
    assert_eq!(d.value()[0].url, "a");
    assert_eq!(d.value()[1].url, "b");
  }

  #[test]
  fn test_unshift_prepends() {
    let d = Rc::new(Downloads::default());
    let d = d.reduce(DownloadAction::Push(make_download("a")));
    let d = d.reduce(DownloadAction::UnShift(make_download("b")));
    assert_eq!(d.value().len(), 2);
    assert_eq!(d.value()[0].url, "b", "unshift should prepend");
    assert_eq!(d.value()[1].url, "a");
  }

  #[test]
  fn test_update_existing() {
    let d = Rc::new(Downloads::default());
    let d = d.reduce(DownloadAction::Push(make_download("url1")));
    let d = d.reduce(DownloadAction::Update(DownloadProgress {
      url: "url1".to_string(),
      percent: 0.5,
      status: ImageState::Loaded,
    }));
    assert_eq!(
      d.value()[0].percent,
      50.0,
      "percent should be scaled by 100"
    );
    assert_eq!(d.value()[0].status, ImageState::Loaded);
  }

  #[test]
  fn test_update_nonexistent_url_is_noop() {
    let d = Rc::new(Downloads::default());
    let d = d.reduce(DownloadAction::Push(make_download("url1")));
    let d = d.reduce(DownloadAction::Update(DownloadProgress {
      url: "url_missing".to_string(),
      percent: 1.0,
      status: ImageState::Error,
    }));
    assert_eq!(d.value()[0].percent, 0.0, "should not change");
    assert_eq!(d.value()[0].status, ImageState::Pending);
  }

  #[test]
  fn test_from_vec() {
    let items = vec![make_download("a"), make_download("b")];
    let d = Downloads::from(items);
    assert_eq!(d.value().len(), 2);
  }
}
