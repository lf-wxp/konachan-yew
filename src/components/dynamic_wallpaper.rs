use bounce::use_atom_value;
use stylist::{self, style};
use web_sys::HtmlCanvasElement;
use yew::{function_component, html, use_effect_with, use_node_ref, Html};

use crate::{store::Theme, 
  utils::{style, PointLine}}
;

#[function_component]
pub fn DynamicWallpaper() -> Html {
  let class_name = get_class_name();
  let theme = use_atom_value::<Theme>();
  let canvas_ref = use_node_ref();
  let canvas_ref_clone = canvas_ref.clone();

  use_effect_with((), move |_| {
    if let Some(canvas) = canvas_ref_clone.cast::<HtmlCanvasElement>() {
      PointLine::new(canvas, 5, "#ccc".to_string(), "#000".to_string());
    }
  });

  html! {
    <section class={class_name}>
      <canvas ref={canvas_ref} class={format!("{theme}")}  />
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
