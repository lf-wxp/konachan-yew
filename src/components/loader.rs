use bounce::use_atom_value;
use stylist::{self, style};
use yew::prelude::*;

use crate::{
  store::Loading,
  utils::style,
};

#[function_component]
pub fn Loader() -> Html {
  let class_name = get_class_name();
  let loading = use_atom_value::<Loading>();
  html! {
    if *loading.value() {
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
      inline-size: 100%;
      block-size: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      .loader {
        inline-size: 50px;
        aspect-ratio: 1.154;
        display: grid;
        color: #000;
        background:
          linear-gradient(to bottom left ,#0000 calc(50% - 1px),var(--theme-base-color) 0 calc(50% + 1px),#0000 0) right/50% 100%,
          linear-gradient(to bottom right,#0000 calc(50% - 1px),var(--theme-primary-color) 0 calc(50% + 1px),#0000 0) left /50% 100%,
          linear-gradient(var(--theme-ancillary-color) 0 0) bottom/100% 2px;
        background-repeat: no-repeat;
        transform-origin: 50% 66%;
        animation: l5 4s infinite linear;
      }
      .loader::before,
      .loader::after {
        content: "";
        grid-area: 1/1;
        background: inherit;
        transform-origin: inherit;
        animation: inherit;
      }
      .loader::after {
        animation-duration: 2s;
      }
      @keyframes l5{
        100% {transform:rotate(1turn)}
      } 
    "#
  ))
}
