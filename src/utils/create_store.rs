//! Macro for creating store types with default values
//!
//! This macro creates a newtype wrapper around an inner type, implementing
//! common traits like `Default`, `Display`, `Clone`, `PartialEq`, and `bounce::Atom`.

/// Creates a new store type with the specified name, inner type, and default value.
///
/// # Arguments
///
/// * `$name` - The name of the struct to create
/// * `$inner_type` - The type of the inner value
/// * `$default_value` - The default value for the type
///
/// # Example
///
/// ```rust
/// create_store!(Security, bool, true);
/// ```
#[macro_export]
macro_rules! create_store {
  ($name:ident, $inner_type:ty, $default_value:expr) => {
    #[derive(PartialEq, bounce::Atom, Clone)]
    pub struct $name($inner_type);

    impl $name {
      #[allow(dead_code)]
      pub fn new(val: $inner_type) -> Self {
        $name(val)
      }
      #[allow(dead_code)]
      pub fn value(&self) -> &$inner_type {
        &self.0
      }
    }

    impl std::fmt::Display for $name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
      }
    }

    impl Default for $name {
      fn default() -> Self {
        $name($default_value)
      }
    }
  };
}
