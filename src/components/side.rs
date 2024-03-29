use stylist::{self, style};
use yew::prelude::*;
use yew_icons::IconId;

use crate::utils::style;

#[derive(Debug, Clone)]
struct Nav {
  icon: IconId,
}

#[function_component]
pub fn Side() -> Html {
  let class_name = get_class_name();

  html! {
    <side class={class_name}>
      <div class="side-nav">
      </div>
    </side>
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
        inline-size: 100%;
        block-size: 100%;
        display: flex;
        flex-flow: column nowrap;
        justify-content: center;
        align-items: center;
    "#
  ))
}
