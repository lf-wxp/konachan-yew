use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;

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
