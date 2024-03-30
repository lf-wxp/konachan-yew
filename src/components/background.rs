use bounce::use_atom_value;
use stylist::{self, style};
use web_sys::HtmlCanvasElement;
use yew::{function_component, html, use_effect_with, use_node_ref, Html};

use crate::{store::Theme, 
  utils::{style, PointLine}}
;

#[function_component]
pub fn Background() -> Html {
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
    <div class={class_name}>
      <canvas ref={canvas_ref} class={format!("{theme}")}  />
    </div>
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
        canvas.light {
          background-image: linear-gradient(to top, #f3e7e9 0%, #e3eeff 99%, #e3eeff 100%);
        }
        canvas.dark {
          background: linear-gradient(110.6deg, rgb(156, 116, 129) -18.3%, rgb(67, 54, 74) 16.4%, rgb(47, 48, 67) 68.2%, rgb(27, 23, 36) 99.1%);
        }
    "#
  ))
}
