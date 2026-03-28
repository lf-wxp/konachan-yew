use crate::{
  model::DownloadProgress,
  store::{DownloadAction, Downloads, use_slice},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::listen_progress;

#[hook]
pub fn use_listen_progress() {
  let downloads = use_slice::<Downloads>();
  let downloads_clone = downloads.clone();
  use_effect_with((), move |_| {
    let downloads_clone = downloads_clone.clone();
    spawn_local(async move {
      let _ = listen_progress(&|data: DownloadProgress| {
        downloads_clone.dispatch(DownloadAction::Update(data));
      })
      .await;
    });
  });
}
