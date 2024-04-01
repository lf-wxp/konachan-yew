use stylist::{self, style};
use yew::{function_component, html, Html};

use crate::utils::{random, style};

#[function_component]
pub fn Background() -> Html {
  let class_name = get_class_name();
  let image_idx = random(0..14);
  let image = format!("/image/bg{}.jpg", image_idx);

  html! {
    <section class={class_name}>
      <img src={image}/>
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
        img {
          width: 100%;
          height: 100%;
          object-fit: cover;
        }
    "#
  ))
}
