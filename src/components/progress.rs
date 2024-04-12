use std::{cell::RefCell, rc::Rc};

use bounce::use_atom_value;
use stylist::{self, style};

use web_sys::{HtmlCanvasElement, HtmlImageElement};
use yew::prelude::*;
use yew::{function_component, use_mut_ref, use_node_ref, Html};

use crate::{
  store::{ImageState, ThemeColor},
  utils::{style, ParticleProgress},
};

#[derive(Properties, PartialEq)]
pub struct Props {
  pub percent: f32,
  pub status: ImageState,
  pub image: String,
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
      }
    }
  });
  html! {
    <div class={class_name}>
      <img src={props.image.clone()} onload={onload} ref={img_ref} />
      <canvas ref={canvas_ref} />
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
    "#
  ))
}
