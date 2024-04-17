use stylist::{self, style};
use yew::prelude::*;

use crate::utils::style;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub visible: bool,
}

#[function_component]
pub fn Pending(props: &Props) -> Html {
  let class_name = get_class_name();
  html! {
    if props.visible {
      <section class={class_name}>
        <div class="loader"/>
      </section>
    }
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
      position: absolute;
      left: 0;
      top: 0;
      right: 0;
      bottom: 0;
      margin: auto;
      inline-size: 32px;
      block-size: 16px;
      .loader {
        inline-size: 32px;
        block-size: 16px;
        display: flex;
        animation: l9-0 1s infinite;
      }
      .loader:before,
      .loader:after {
        content: "";
        flex: 1;
        background: var(--theme-base-color);
        transform-origin: top right;
        animation: l9-1 1s infinite;
      }
      .loader:after {
        background: var(--theme-primary-color);
        transform-origin: top left;
        --s:-1;
      }
      @keyframes l9-0 {
         100% {transform: translateY(100%)}
      }
      @keyframes l9-1 {
         100% {transform: rotate(calc(var(--s,1)*90deg))}
      }
    "#
  ))
}
