use futures_util::StreamExt;
use gloo_console::log;
use gloo_net::http::{Headers, Request};
use js_sys::encode_uri;
use tauri_sys::core;
use tauri_sys::event::listen;

use crate::{
  model::{
    Action, DownloadParam, DownloadProgress, Empty, Error, FetchParams, ImageData, ImageRes,
  },
  utils::download_file,
};

pub async fn fetch_action(params: FetchParams) -> Result<ImageRes, Error> {
  #[cfg(any(feature = "web", feature = "safe"))]
  {
    let url = "/api/post";
    let query = params.param();
    let headers = Headers::new();
    headers.append("x-api-key", "konachan-api");
    headers.append("ContentType", "application/json");
    let resp = Request::get(url)
      .headers(headers)
      .query(query)
      .send()
      .await?;
    let json: ImageRes = resp.json().await?;
    return Ok(json);
  }
  #[cfg(feature = "tauri")]
  {
    let json: ImageRes = core::invoke::<ImageRes>(&Action::GetPost.to_string(), &params).await;
    return Ok(json);
  }
  #[cfg(feature = "fake")]
  {
    let json_data = include_str!("../../static/mock/post.json");
    let json: ImageRes = serde_json::from_str(json_data)?;
    return Ok(json);
  }
  Ok(ImageRes {
    code: 0,
    msg: None,
    data: ImageData {
      count: 0,
      images: vec![],
    },
  })
}

pub async fn download_action(url: &str, name: &str) -> Result<(), Error> {
  #[cfg(not(feature = "tauri"))]
  {
    let url = format!("/api/image?url={}", encode_uri(url));
    download_file(&url, name);
    Ok(())
  }
  #[cfg(feature = "tauri")]
  {
    core::invoke::<()>(
      &Action::DownloadImage.to_string(),
      &DownloadParam {
        url: url.to_string(),
      },
    )
    .await;
    Ok(())
  }
}

pub async fn close_splashscreen() -> Result<(), Error> {
  #[cfg(feature = "tauri")]
  {
    core::invoke::<()>(&Action::CloseSplashscreen.to_string(), &Empty).await;
    return Ok(());
  }
  Ok(())
}

pub async fn listen_progress(callback: &dyn Fn(DownloadProgress)) {
  #[cfg(feature = "tauri")]
  {
    let mut events = listen::<DownloadProgress>("progress").await.unwrap();
    while let Some(event) = events.next().await {
      log!("progress", format!("{:?}", event.payload));
      callback(event.payload);
    }
  }
}
