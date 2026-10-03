//! Runtime configuration injected by the hosting environment.
//!
//! A single WASM bundle can serve several deployment profiles by reading a
//! global JS object (`window.__KONACHAN_CONFIG__`) that is populated by
//! `/config.js` before the bundle boots. In Docker that file is regenerated
//! from environment variables on every container start, so one image can run
//! both the regular and the safe (content-filtered) mode.
//!
//! When a key is missing the compile-time cargo feature is used as the
//! default, which keeps `--features safe` builds working with no extra config.

use js_sys::Reflect;
use wasm_bindgen::JsValue;

/// Global populated by `/config.js` before the WASM bundle is initialised.
const GLOBAL_KEY: &str = "__KONACHAN_CONFIG__";

/// Runtime flags resolved from the injected configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeConfig {
  /// Safe mode hides the content-filter toggle so sensitive images are always
  /// filtered. Defaults to the compile-time `safe` cargo feature.
  pub safe: bool,
}

// The default depends on the compile-time `safe` feature, so it cannot be
// derived.
#[allow(clippy::derivable_impls)]
impl Default for RuntimeConfig {
  fn default() -> Self {
    Self {
      safe: cfg!(feature = "safe"),
    }
  }
}

impl RuntimeConfig {
  /// Read the injected configuration, falling back to compile-time defaults.
  #[must_use]
  fn load() -> Self {
    let mut config = Self::default();
    if let Some(safe) = injected_bool("safe") {
      config.safe = safe;
    }
    config
  }
}

/// Resolve the runtime configuration for the current page.
#[must_use]
pub fn runtime_config() -> RuntimeConfig {
  RuntimeConfig::load()
}

/// Whether safe mode is active for the current page.
#[must_use]
pub fn is_safe() -> bool {
  runtime_config().safe
}

/// Read a boolean field from the injected global object.
fn injected_bool(key: &str) -> Option<bool> {
  let window: JsValue = web_sys::window()?.into();
  let global = Reflect::get(&window, &JsValue::from_str(GLOBAL_KEY)).ok()?;
  if global.is_undefined() || global.is_null() {
    return None;
  }
  Reflect::get(&global, &JsValue::from_str(key))
    .ok()?
    .as_bool()
}
