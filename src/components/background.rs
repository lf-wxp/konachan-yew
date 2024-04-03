use bounce::use_atom_setter;
use stylist::{self, style};
use web_sys::HtmlImageElement;
use yew::{function_component, html, Callback, Event, Html};

use crate::{
  store::ThemeColor,
  utils::{bare_rgb, get_html_image_to_vec, get_target, random, style},
};

#[function_component]
pub fn Background() -> Html {
  let class_name = get_class_name();
  let image_idx = random(0..14);
  let theme_color = use_atom_setter::<ThemeColor>();
  let image = format!("/image/bg{}.jpg", image_idx);

  let load = Callback::from(move |e: Event| {
    if let Some(target) = get_target::<Event, HtmlImageElement>(e) {
      let pixel = get_html_image_to_vec(target).unwrap_or(vec![]);
      let colors = color_thief::get_palette(&pixel, color_thief::ColorFormat::Rgb, 10, 3).unwrap();
      let theme = colors[0];
      let primary = colors[1];
      let ancillary = colors[2];
      theme_color(ThemeColor::new(
        bare_rgb(theme),
        bare_rgb(primary),
        bare_rgb(ancillary),
      ));
    }
  });

  html! {
    <section class={class_name}>
      <img src={image} onload={load}/>
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
