use bounce::use_atom_value;
use stylist::{self, style};
use yew::prelude::*;

use crate::{components::Progress, store::Downloads, utils::style};

#[function_component]
pub fn DownloadList() -> Html {
  let class_name = get_class_name();
  let downloads = use_atom_value::<Downloads>();
  html! {
    <section class={class_name}>
      <ul class="scroll-bar">
        {for downloads.value().iter().map(|item| {
        html!{
          <li class="bk-download-item">
            <Progress status={item.status.clone()} percent={item.percent} image={item.preview.clone()}  />
          </li>
        }})}
      </ul>
    </section>
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
      .bk-download-item {
        aspect-ratio: 16 / 9;
        position: relative;
        margin-block-end: 10px;
        border-radius: 2px;
        overflow: hidden;
      }
      .bk-download-image {
        block-size: 100%;
        inline-size: 100%;
        object-fit: cover;
      }
    "#
  ))
}
