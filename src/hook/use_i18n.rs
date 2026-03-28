use crate::utils::{YewI18n, use_i18n as use_i18n_hook};
use yew::prelude::*;

#[hook]
pub fn use_i18n() -> YewI18n {
  let i18n = use_i18n_hook();
  YewI18n {
    language: i18n.language.clone(),
    translations: i18n.translations.clone(),
  }
}
