use bounce::use_atom_value;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{store::ThemeColor, utils::query_selector};

#[hook]
pub fn use_theme() {
  let theme_color = use_atom_value::<ThemeColor>();
  use_effect_with(theme_color, |theme_color| {
    if let Some(element) = query_selector::<HtmlElement>("html") {
      let css_text = theme_color.get_css_text();
      element.style().set_css_text(&css_text);
    }
  });
}
