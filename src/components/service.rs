use crate::store::{use_atom_setter, use_atom_value, use_slice_dispatch, use_slice_value};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::{Html, function_component, html, use_effect_with};

#[cfg(feature = "tauri")]
use crate::hook::use_listen_progress;
use crate::{
  components::{NoticeTag, use_notify},
  hook::{use_i18n, use_theme},
  model::FetchParams,
  store::{Images, Loading, Mode, Page, PageAction, Refresh, Tags},
  utils::fetch_action,
};

#[function_component]
pub fn Service() -> Html {
  use_theme();
  let page_dispatch = use_slice_dispatch::<Page>();
  let page = use_slice_value::<Page>();
  let loading_handle = use_atom_setter::<Loading>();
  let refresh = use_atom_value::<Refresh>();
  let mode = use_atom_value::<Mode>();
  let tags = use_atom_value::<Tags>();
  let images_handle = use_atom_setter::<Images>();
  let notify = use_notify();
  let i18n = use_i18n();

  #[cfg(feature = "tauri")]
  {
    use_listen_progress();
  }
  use_effect_with(
    (page.current, tags, mode, refresh),
    move |val: &(usize, Rc<Tags>, Rc<Mode>, Rc<Refresh>)| {
      let (current, tags, mode, _) = val;
      let current = *current;
      let tags = (**tags).clone();
      let mode = (**mode).clone();
      loading_handle.emit(Loading::new(true));
      spawn_local(async move {
        let result =
          fetch_action(FetchParams::new(current as u32, tags.value().clone(), mode)).await;
        loading_handle.emit(Loading::new(false));
        match result {
          Ok(res) => {
            images_handle.emit(Images::from(res.data.images));
            page_dispatch.emit(PageAction::Total(res.data.count));
          }
          Err(e) => {
            web_sys::console::log_1(&format!("[Service] Error: {:?}", e).into());
            notify(i18n.t("get list error"), NoticeTag::Danger, Some(3));
          }
        };
      });
    },
  );

  html! {}
}
