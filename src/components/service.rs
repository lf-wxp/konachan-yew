use bounce::{use_atom, use_atom_value};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with, Html};

use crate::{
  hook::use_theme,
  model::FetchParams,
  store::{Images, Page},
  utils::fetch_data,
};

#[function_component]
pub fn Service() -> Html {
  use_theme();
  let page = use_atom_value::<Page>();
  let images_handle = use_atom::<Images>();
  let page_clone = page.clone();

  use_effect_with(page, move |_| {
    spawn_local(async move {
      let res = fetch_data(FetchParams::new(page_clone.value(), None, None, None))
        .await
        .unwrap();
      images_handle.set(Images::from(res.data.images));
    });
  });

  html! {}
}
