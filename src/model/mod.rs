
pub type Error = Box<dyn std::error::Error>;
pub type UResult<T> = std::result::Result<T, Error>;
#[derive(PartialEq, Clone)]
pub struct Option<T = String> {
  pub label: String,
  pub value: T,
}
