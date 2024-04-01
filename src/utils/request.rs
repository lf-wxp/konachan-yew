use gloo_net::http::{Headers, Request};
use tauri_sys::tauri;

use crate::model::{Action, Error, FetchParams, ImageRes};

pub async fn fetch_data(params: FetchParams) -> Result<ImageRes, Error> {
  #[cfg(feature = "web")]
  {
    let url = "/api/post";
    let query = params.param();
    let headers = Headers::new();
    headers.append("x-api-key", "konachan-api");
    headers.append("ContentType", "application/json");
    let resp = Request::get(&url)
      .headers(headers)
      .query(query)
      .send()
      .await?;
    let json: ImageRes = resp.json().await?;
    return Ok(json);
  }
  #[cfg(feature = "tauri")]
  {
    let json_data: String = tauri::invoke(&Action::GetPost.to_string(), &params).await?;
    let json: ImageRes = serde_json::from_str(&json_data)?;
    return Ok(json);
  }
  #[cfg(feature = "fake")]
  {
    let json_data = include_str!("../../static/mock/post.json");
    let json: ImageRes = serde_json::from_str(&json_data)?;
    Ok(json)
  }
}
