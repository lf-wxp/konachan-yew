
#[macro_export]
macro_rules! create_store {
  ($name:ident, $inner_type:ty, $default_value:expr) => {
    #[derive(PartialEq, bounce::Atom, Clone)]
    pub struct $name($inner_type);

    impl $name {
      pub fn new(val: $inner_type) -> Self {
        $name(val)
      }
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
