#[cfg(any(feature = "web", feature = "safe"))]
use gloo_net::http::{Headers, Request};
#[cfg(not(feature = "tauri"))]
use {crate::model::DownloadProgress, crate::utils::download_file, js_sys::encode_uri};
#[cfg(feature = "tauri")]
use {
  crate::model::{Action, DownloadParam, DownloadProgress, Empty},
  futures_util::StreamExt,
  gloo_console::log,
  tauri_sys::core,
  tauri_sys::event::listen,
};

use crate::model::{Error, FetchParams, ImageRes};

pub async fn fetch_action(_params: FetchParams) -> Result<ImageRes, Error> {
  #[cfg(any(feature = "web", feature = "safe"))]
  {
    let url = "/api/post";
    let query = _params.param();
    let headers = Headers::new();
    headers.append("x-api-key", "konachan-api");
    headers.append("ContentType", "application/json");
    let resp = Request::get(url)
      .headers(headers)
      .query(query)
      .send()
      .await?;
    let json: ImageRes = resp.json().await?;
    Ok(json)
  }

  #[cfg(all(feature = "tauri", not(any(feature = "web", feature = "safe"))))]
  {
    let json: ImageRes = core::invoke::<ImageRes>(&Action::GetPost.to_string(), &_params).await;
    Ok(json)
  }

  #[cfg(all(
    feature = "fake",
    not(any(feature = "web", feature = "safe")),
    not(feature = "tauri")
  ))]
  {
    let json_data = include_str!("../../static/mock/post.json");
    let json: ImageRes = serde_json::from_str(json_data)?;
    Ok(json)
  }

  #[cfg(not(any(feature = "web", feature = "safe", feature = "tauri", feature = "fake")))]
  {
    Err("No feature enabled. Please enable one of: web, safe, tauri, fake".into())
  }
}

pub async fn download_action(
  url: &str,
  #[allow(unused_variables)] name: &str,
) -> Result<(), Error> {
  #[cfg(not(feature = "tauri"))]
  {
    let url = format!("/api/image?url={}", encode_uri(url));
    download_file(&url, name).map_err(|e| format!("{:?}", e))?;
    Ok(())
  }

  #[cfg(feature = "tauri")]
  {
    let _ = core::invoke::<()>(
      &Action::DownloadImage.to_string(),
      &DownloadParam {
        url: url.to_string(),
      },
    )
    .await;
    Ok(())
  }
}

#[allow(dead_code)]
pub async fn close_splashscreen() -> Result<(), Error> {
  #[cfg(feature = "tauri")]
  {
    let _ = core::invoke::<()>(&Action::CloseSplashscreen.to_string(), &Empty).await;
  }
  Ok(())
}

#[cfg(feature = "tauri")]
pub async fn listen_progress(callback: &dyn Fn(DownloadProgress)) -> Result<(), Error> {
  let mut events = listen::<DownloadProgress>("progress").await?;
  while let Some(event) = events.next().await {
    log!("progress", format!("{:?}", event.payload));
    callback(event.payload);
  }
  Ok(())
}

#[cfg(not(feature = "tauri"))]
#[allow(dead_code)]
pub async fn listen_progress(_callback: &dyn Fn(DownloadProgress)) -> Result<(), Error> {
  // No-op for non-tauri builds
  Ok(())
}
