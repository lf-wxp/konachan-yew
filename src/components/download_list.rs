use bounce::use_slice_value;
use stylist::{self, style};
use yew::prelude::*;

use crate::{
  components::Progress,
  store::{Downloads, ImageState},
  utils::style,
};

#[function_component]
pub fn DownloadList() -> Html {
  let class_name = get_class_name();
  let downloads = use_slice_value::<Downloads>();
  let class = |status: ImageState| format!("bk-download-item {}", status);

  html! {
    <section class={class_name}>
      <ul class="scroll-bar">
        {for downloads.value().iter().map(|item| {
          html!{
            <li class={class(item.status.clone())}>
              <Progress status={item.status.clone()} percent={item.percent} image={item.preview.clone()} url={item.url.clone()}  />
            </li>
          }})
        }
      </ul>
    </section>
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
      height: calc(100% - 205px);
      margin-block-end: 0;
      .scroll-bar {
        overflow-y: auto;
        height: 100%;
      }
      .bk-download-item {
        aspect-ratio: 16 / 9;
        position: relative;
        margin-block-end: 10px;
        border-radius: 2px;
        overflow: hidden;
      }
      .bk-download-item.error {
        outline: 1px solid var(--danger-color);
      }
      .bk-download-image {
        block-size: 100%;
        inline-size: 100%;
        object-fit: cover;
      }
    "#
  ))
}
