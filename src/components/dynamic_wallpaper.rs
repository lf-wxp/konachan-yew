use std::{cell::RefCell, rc::Rc};

use bounce::use_atom_value;
use stylist::{self, style};
use web_sys::HtmlCanvasElement;
use yew::{function_component, html, use_effect_with, use_mut_ref, use_node_ref, Html};

use crate::{
  store::ThemeColor,
  utils::{style, PointLine},
};

#[function_component]
pub fn DynamicWallpaper() -> Html {
  let class_name = get_class_name();
  let theme_color = use_atom_value::<ThemeColor>();
  let canvas_ref = use_node_ref();
  let canvas_ref_clone = canvas_ref.clone();
  let point_line: Rc<RefCell<Option<Rc<RefCell<PointLine>>>>> = use_mut_ref(|| {
    Some(PointLine::new(
      None,
      5,
      "#ccc".to_string(),
      "#000".to_string(),
    ))
  });

  let point_line_clone = point_line.clone();
  use_effect_with((), move |_| {
    if let Some(canvas) = canvas_ref_clone.cast::<HtmlCanvasElement>() {
      if let Some(point_line) = &*point_line_clone.borrow() {
        (*point_line.borrow_mut()).set_canvas(canvas);
        (*point_line.borrow_mut()).init();
      }
    }
  });

  use_effect_with(theme_color, move |color: &Rc<ThemeColor>| {
    if let Some(point_line) = &*point_line.borrow() {
      let (theme_color, primary_color, __) = color.get_color();
      (*point_line.borrow_mut()).set_color(theme_color, primary_color);
    }
  });

  html! {
    <section class={class_name}>
      <canvas ref={canvas_ref}  />
    </section>
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
        block-size: 100%;
        inline-size: 100%;
        position: fixed;
        z-index: -1;
        inset-block: 0;

        canvas {
          block-size: 100%;
          inline-size: 100%;
          transition: background 0.2s ease;
        }
    "#
  ))
}
