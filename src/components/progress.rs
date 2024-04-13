use bounce::use_atom_value;
use std::{cell::RefCell, rc::Rc};
use stylist::{self, style};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, HtmlImageElement};
use yew::prelude::*;
use yew::{function_component, use_mut_ref, use_node_ref, Html};
use yew_icons::{Icon, IconId};

use crate::{
  store::{ImageState, ThemeColor},
  utils::{download_action, style, ParticleProgress},
};

#[derive(Properties, PartialEq)]
pub struct Props {
  pub percent: f32,
  pub status: ImageState,
  pub image: String,
  pub url: String,
}

#[function_component]
pub fn Progress(props: &Props) -> Html {
  let class_name = get_class_name();
  let canvas_ref = use_node_ref();
  let img_ref = use_node_ref();
  let theme_color = use_atom_value::<ThemeColor>();
  let particle_progress: Rc<RefCell<Option<Rc<RefCell<ParticleProgress>>>>> =
    use_mut_ref(|| Some(ParticleProgress::new(None, None, "red".to_string())));
  let canvas_ref_clone = canvas_ref.clone();
  let img_ref_clone = img_ref.clone();
  let particle_progress_clone = particle_progress.clone();
  let particle_progress_retry = particle_progress.clone();
  let url = props.url.clone();
  use_effect_with(props.percent, move |percent: &f32| {
    if let Some(progress) = &*particle_progress_clone.borrow() {
      (*progress.borrow_mut()).set_percent(*percent as f64);
    }
  });
  let particle_progress_clone = particle_progress.clone();
  use_effect_with(theme_color, move |color: &Rc<ThemeColor>| {
    let (theme_color, _, _) = color.get_color();
    if let Some(progress) = &*particle_progress_clone.borrow() {
      (*progress.borrow_mut()).set_color(theme_color);
    }
  });
  let particle_progress_clone = particle_progress.clone();
  use_effect_with(props.status.clone(), move |status: &ImageState| {
    if matches!(status, ImageState::Error) {
      if let Some(progress) = &*particle_progress_clone.borrow() {
        (*progress.borrow_mut()).stop();
      }
    }
  });
  let onload = Callback::from(move |_: Event| {
    if let Some(canvas) = canvas_ref_clone.cast::<HtmlCanvasElement>() {
      if let Some(progress) = &*particle_progress.borrow() {
        (*progress.borrow_mut()).set_canvas(canvas);
      }
    }
    if let Some(img) = img_ref_clone.cast::<HtmlImageElement>() {
      if let Some(progress) = &*particle_progress.borrow() {
        (*progress.borrow_mut()).set_image(img);
        (*progress.borrow_mut()).init();
        (*progress.borrow_mut()).start();
      }
    }
  });
  let retry = {
    let particle_progress_retry = particle_progress_retry.clone();
    Callback::from(move |url: String| {
      let particle_progress_retry = particle_progress_retry.clone();
      spawn_local(async move {
        if let Some(progress) = &*particle_progress_retry.borrow() {
          (*progress.borrow_mut()).start();
        }
        let _ = download_action(&url, "").await;
      });
    })
  };

  html! {
    <div class={class_name}>
      <img src={props.image.clone()} onload={onload} ref={img_ref} />
      <canvas ref={canvas_ref} />
      if matches!(props.status, ImageState::Error) {
        <Icon
          class="retry"
          icon_id={IconId::FeatherRefreshCcw}
          width="1em"
          height="1em"
          onclick={retry.reform(move |_| url.clone())}
        />
      }
    </div>
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
      inline-size: 100%;
      block-size: 100%;
      overflow: hidden;
      position: relative;
      img {
        inline-size: 100%;
        block-size: 100%;
        opacity: 0.2;
        filter: blur(1px);
      }
      canvas {
        inline-size: 100%;
        block-size: 100%;
        position: absolute;
        inset-block-start: 0;
        inset-inline-start: 0;
      }
      .retry {
        position: absolute;
        inset-block-start: 5px;
        inset-inline-end: 5px;
        cursor: pointer;
        color: var(--danger-color);
      }
    "#
  ))
}
