use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

lazy_static! {
  pub static ref TRANSLATIONS: HashMap<String, Value> = {
    let mut translations = HashMap::new();
    translations.insert(
      "en".to_string(),
      serde_json::json!({
          "get list error": "Get image list error, try again!",
      }),
    );
    translations.insert(
      "zh".to_string(),
      serde_json::json!({
          "get list error": "获取图片列表失败，请重试",
      }),
    );
    translations
  };
}

/// I18n context
#[derive(Clone, PartialEq)]
pub struct I18n {
  pub language: String,
  pub translations: HashMap<String, Value>,
}

#[allow(dead_code)]
impl I18n {
  pub fn new(language: String, translations: HashMap<String, Value>) -> Self {
    Self {
      language,
      translations,
    }
  }

  pub fn t(&self, key: &str) -> String {
    self
      .translations
      .get(&self.language)
      .and_then(|v| v.get(key))
      .and_then(|v| v.as_str())
      .unwrap_or(key)
      .to_string()
  }

  pub fn set_language(&mut self, language: String) {
    self.language = language;
  }
}

impl Default for I18n {
  fn default() -> Self {
    Self {
      language: "en".to_string(),
      translations: HashMap::new(),
    }
  }
}

/// Props for I18nProvider
#[derive(Properties, PartialEq)]
pub struct I18nProviderProps {
  pub supported_languages: Vec<&'static str>,
  pub translations: HashMap<String, Value>,
  #[prop_or_default]
  pub children: Children,
}

/// I18n provider component
#[function_component]
pub fn I18nProvider(props: &I18nProviderProps) -> Html {
  // Detect browser language
  let default_language = if let Some(window) = web_sys::window() {
    window
      .navigator()
      .language()
      .map(|s| s.split_at(2).0.to_string())
      .unwrap_or("en".to_string())
  } else {
    "en".to_string()
  };

  // Check if default language is supported, otherwise use "en"
  let language = if props
    .supported_languages
    .contains(&default_language.as_str())
  {
    default_language
  } else {
    "en".to_string()
  };

  let i18n = use_state(|| I18n::new(language, props.translations.clone()));

  html! {
      <ContextProvider<Rc<I18n>> context={Rc::new((*i18n).clone())}>
          {props.children.clone()}
      </ContextProvider<Rc<I18n>>>
  }
}

/// Hook to get i18n context
#[hook]
pub fn use_i18n() -> Rc<I18n> {
  use_context::<Rc<I18n>>().expect("I18nProvider not found")
}

/// Hook to get translation
#[hook]
pub fn use_translation() -> YewI18n {
  let i18n = use_i18n();
  YewI18n {
    language: i18n.language.clone(),
    translations: i18n.translations.clone(),
  }
}

/// YewI18n struct for compatibility
#[derive(Clone, PartialEq)]
pub struct YewI18n {
  pub language: String,
  pub translations: HashMap<String, Value>,
}

impl YewI18n {
  pub fn t(&self, key: &str) -> String {
    self
      .translations
      .get(&self.language)
      .and_then(|v| v.get(key))
      .and_then(|v| v.as_str())
      .unwrap_or(key)
      .to_string()
  }

  #[allow(dead_code)]
  pub fn set_translation_language(&mut self, language: &str) -> Result<(), ()> {
    self.language = language.to_string();
    Ok(())
  }
}
