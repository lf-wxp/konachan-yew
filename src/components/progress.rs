use gloo_console::log;
use stylist::{self, style};
use yew::prelude::*;

use crate::{store::ImageState, utils::style};

#[derive(Properties, PartialEq)]
pub struct Props {
  pub percent: f32,
  pub status: ImageState,
}

#[function_component]
pub fn Progress(props: &Props) -> Html {
  let class_name = get_class_name();
  let load_statue = |i: u32| {
    let percent = props.percent * 100.0;
    let quotient = percent.div_euclid(5.0);
    let remainder = percent.rem_euclid(5.0);
    log!("progress", quotient, remainder);
    if i as f32 <= quotient {
      return "loaded";
    }
    if (remainder >= 0.0 && i as f32 == quotient + 1.0) || (percent == 0.0 && i == 1) {
      return "pending";
    }
    ""
  };
  let class = |i: u32| format!("{} bk-progress bk-progress-{}", load_statue(i), i);

  html! {
    <div class={class_name}>
      {for (1..21).map(|item| {
        html!{
          <span class={class(item)} />
        }
      })}
    </div>
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
      display: grid;
      grid-template-columns: repeat(5, 1fr);
      grid-template-rows: repeat(4, 1fr);
      gap: 0px;
      .bk-progress {
        background: var(--theme-base-color);
        opacity: 0.5;
      }
      .bk-progress.pending {
        animation: fade 1s ease-in-out 0s infinite alternate;
      }
      .bk-progress.loaded {
        opacity: 0;
      }
      @keyframes fade {
         0% { opacity: 0.5 }
         100% { opacity: 0 }
      }
    "#
  ))
}
