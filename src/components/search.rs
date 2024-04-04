use bounce::use_atom_setter;
use stylist::{self, style};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
  store::{Page, Tags, Total},
  utils::{get_target, style},
};

#[function_component]
pub fn Search() -> Html {
  let class_name = get_class_name();
  let tags = use_atom_setter::<Tags>();
  let page = use_atom_setter::<Page>();
  let total = use_atom_setter::<Total>();
  let value = use_state(|| "".to_string());
  let value_clone = value.clone();

  let oninput = Callback::from(move |e: InputEvent| {
    if let Some(target) = get_target::<InputEvent, HtmlInputElement>(e) {
      value_clone.set(target.value());
    }
  });

  let onkeypress = {
    let value_clone = value.clone();
    Callback::from(move |_: KeyboardEvent| {
      total(Total::new(0));
      page(Page::new(1));
      tags(Tags::new((*value_clone).clone()));
    })
  };

  html! {
  <input
    class={class_name}
    value={(*value).clone()}
    oninput={oninput}
    onkeypress={onkeypress}
  />
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
      inline-size: 100%;
      box-sizing: border-box;
      height: 32px;
      background: transparent;
      border-right: 4px solid var(--theme-base-color);
      border-left: 4px solid var(--theme-base-color);
      border-top: none;
      border-bottom: none;
      color: var(--theme-base-color);
      padding: 0 5px;
      outline: none;
      margin-block-end: 10px;
    "#
  ))
}
