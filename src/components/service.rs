use bounce::{use_atom_setter, use_atom_value};
use gloo_console::log;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with, Html};

use crate::{
  components::{use_notify, NoticeTag},
  hook::{use_i18n, use_theme},
  model::FetchParams,
  store::{Images, Loading, Mode, Page, Refresh, Tags, Total},
  utils::{fetch_action, listen_progress},
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
  let notify = use_notify();
  let i18n = use_i18n();

  use_effect_with(
    (page, tags, mode, refresh),
    move |val: &(Rc<Page>, Rc<Tags>, Rc<Mode>, Rc<Refresh>)| {
      let (page, tags, mode, _) = val;
      let page = (**page).clone();
      let tags = (**tags).clone();
      let mode = (**mode).clone();
      loading_handle(Loading::new(true));
      spawn_local(async move {
        match fetch_action(FetchParams::new(*page.value(), tags.value().clone(), mode)).await {
          Ok(res) => {
            images_handle(Images::from(res.data.images));
            total_handle(Total::new(res.data.count as u32));
            log!("loading");
            listen_progress().await;
          }
          Err(_) => {
            notify(i18n.t("get list error"), NoticeTag::Danger, Some(3));
          }
        };
        loading_handle(Loading::new(false));
      });
    },
  );

  html! {}
}
