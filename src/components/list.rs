use bounce::use_atom_value;
use yew::{function_component, html, use_effect_with, Html};
use gloo_console::log;

use crate::store::Images;

#[function_component]
pub fn List() -> Html {
  let images = use_atom_value::<Images>();

  use_effect_with(images, move |text| {
    log!("images", format!("{:?}", text));
  });
  html! { }
}
