pub mod theme;
pub mod page;
pub mod image;
pub mod size;
pub mod security;
pub mod mode;
pub mod loading;
pub mod refresh;
pub mod tags;

pub use theme::*;
pub use page::*;
pub use image::*;
pub use size::*;
pub use security::*;
pub use mode::*;
pub use loading::*;
pub use refresh::*;
pub use tags::*;

pub trait Invertible {
  fn invert(&self) -> Self;
}
