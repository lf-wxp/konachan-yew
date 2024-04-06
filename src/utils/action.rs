use gloo_console::log;
use gloo_net::http::{Headers, Request};
use js_sys::encode_uri;
use tauri_sys::tauri;
use tauri_sys::event::listen;
use futures_util::StreamExt;

use crate::{
  model::{Action, DownloadParam, DownloadProgress, Empty, Error, FetchParams, ImageRes},
  store::{Download, Image},
  utils::download_file,
};

pub async fn fetch_action(params: FetchParams) -> Result<ImageRes, Error> {
  #[cfg(feature = "web")]
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
    let json: ImageRes = tauri::invoke(&Action::GetPost.to_string(), &params).await?;
    return Ok(json);
  }
  #[cfg(feature = "fake")]
  {
    let json_data = include_str!("../../static/mock/post.json");
    let json: ImageRes = serde_json::from_str(json_data)?;
    Ok(json)
  }
}

pub async fn download_action(item: Image) -> Result<(), Error> {
  #[cfg(not(feature = "tauri"))]
  {
    let url = format!("/api/image?url={}", encode_uri(&item.url));
    download_file(&url, &item.name);
    Ok(())
  }
  #[cfg(feature = "tauri")]
  {
    tauri::invoke(
      &Action::DownloadImage.to_string(),
      &DownloadParam { url: item.url },
    )
    .await?;
    Ok(())
  }
}

pub async fn close_splashscreen() -> Result<(), Error> {
  #[cfg(feature = "tauri")]
  {
    tauri::invoke(&Action::CloseSplashscreen.to_string(), &Empty).await?;
    Ok(())
  }
}

pub async fn listen_progress() {
  #[cfg(feature = "tauri")]
  {
    let mut events = listen::<DownloadProgress>("progress").await.unwrap();
    while let Some(event) = events.next().await {
      log!(&format!("Got payload: {:?}", event.payload));
    } 
  }
}
