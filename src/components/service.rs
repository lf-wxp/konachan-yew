use std::rc::Rc;

use bounce::{use_atom_setter, use_atom_value};
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use rand::{seq::SliceRandom, thread_rng};
use yew::{function_component, html, use_effect_with, Html};

use crate::{
  hook::use_theme,
  model::FetchParams,
  store::{Images, Loading, Mode, Page, Refresh, Tags, Total},
  utils::fetch_action,
};

#[function_component]
pub fn Service() -> Html {
  use_theme();
  let page = use_atom_value::<Page>();
  let total_handle = use_atom_setter::<Total>();
  let loading_handle = use_atom_setter::<Loading>();
  let refresh = use_atom_value::<Refresh>();
  let mode = use_atom_value::<Mode>();
  let tags = use_atom_value::<Tags>();
  let images_handle = use_atom_setter::<Images>();

  use_effect_with((page, tags, mode, refresh), move |val: &(Rc<Page>, Rc<Tags>, Rc<Mode>, Rc<Refresh>)| {
    let (page, tags, mode, _) = val;
    let page = (**page).clone();
    let tags = (**tags).clone();
    let mode = (**mode).clone();
    loading_handle(Loading::new(true));
    spawn_local(async move {
      let res = fetch_action(FetchParams::new(*page.value(), tags.value().clone(), mode))
        .await
        .unwrap();
      let mut images = res.data.images;
      let mut rng = thread_rng();
      images.shuffle(&mut rng);
      loading_handle(Loading::new(false));
      images_handle(Images::from(images));
      total_handle(Total::new(res.data.count as u32));
      log!("loading");
    });
  });

  html! {}
}
