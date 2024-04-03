use bounce::{use_atom, use_atom_setter, use_atom_value};
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with, Html};

use crate::{
  hook::use_theme,
  model::FetchParams,
  store::{Refresh, Images, Loading, Page},
  utils::fetch_data,
};

#[function_component]
pub fn Service() -> Html {
  use_theme();
  let page = use_atom_value::<Page>();
  let loading_handle = use_atom_setter::<Loading>();
  let refresh = use_atom_value::<Refresh>();
  let images_handle = use_atom::<Images>();
  let page_clone = page.clone();

  use_effect_with((page, refresh), move |_| {
    loading_handle(Loading::new(true));
    spawn_local(async move {
      let res = fetch_data(FetchParams::new(page_clone.value(), None, None, None))
        .await
        .unwrap();
      loading_handle(Loading::new(false));
      images_handle.set(Images::from(res.data.images));
      log!("loading");
    });
  });

  html! {}
}
