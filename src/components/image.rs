use stylist::{self, style};
use web_sys::HtmlImageElement;
use yew::prelude::*;

use crate::{
  components::Pending,
  store::ImageState,
  utils::{get_target, style},
};

#[derive(Properties, PartialEq)]
pub struct Props {
  pub src: String,
  pub alternative: Option<String>,
  pub class_name: String,
  pub style: String,
  pub height: f64,
  pub width: f64,
}

#[function_component]
pub fn Image(props: &Props) -> Html {
  let class_name = get_class_name();
  let state = use_state(|| ImageState::Pending);
  let animation_end = use_state(|| false);
  let state_clone = state.clone();

  let class = format!("{} {}", props.class_name, *state);
  let figure_class = format!(
    "bk-image {}",
    if *animation_end { "animationend" } else { "" }
  );
  let figure_style = format!(
    "width: {}px; height: {}px",
    props.width - 10.0,
    props.height - 10.0
  );

  let onanimationend = Callback::from(move |_: AnimationEvent| {
    animation_end.set(true);
  });
  let onload = Callback::from(move |_: Event| {
    if !matches!(*state_clone, ImageState::Error) {
      state_clone.set(ImageState::Loaded)
    }
  });
  let onerror = {
    let alternative = props.alternative.clone();
    let state = state.clone();
    Callback::from(move |e: Event| {
      state.set(ImageState::Error);
      if let Some(target) = get_target::<Event, HtmlImageElement>(e) {
        if let Some(src) = &alternative {
          target.set_src(src);
        }
      }
    })
  };
  html! {
    <div class={class_name}>
      <figure class={figure_class} style={figure_style}>
        <img
          src={props.src.clone()}
          style={props.style.clone()}
          class={class}
          onanimationend={onanimationend}
          onerror={onerror}
          onload={onload}
        />
        <Pending visible={matches!(*state, ImageState::Pending)} />
      </figure>
    </div>
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
      @keyframes fadeIn {
        0% {
          opacity: 0;
        }
        100% {
          opacity: 1;
        }
      }
      
      .bk-image {
        border-radius: 5px;
        overflow: hidden;
        margin: 0;
      }
      
      .bk-image.animationend {
        background-color: transparent;
      }
      
      .bk-image > img {
        opacity: 0;
      }
      
      .bk-image > img.loaded,.bk-image > img.error {
        animation: fadeIn 0.2s ease 0s 1 normal both;
      }
      .bk-image > img.error {
        object-fit: scale-down;
      }
    "#
  ))
}
