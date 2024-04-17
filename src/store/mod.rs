pub mod theme;
pub mod page;
pub mod image;
pub mod size;
pub mod security;
pub mod mode;
pub mod loading;
pub mod refresh;
pub mod tags;
pub mod download;

pub(crate) use theme::*;
pub(crate) use page::*;
pub(crate) use image::*;
pub(crate) use size::*;
pub(crate) use security::*;
pub(crate) use mode::*;
pub(crate) use loading::*;
pub(crate) use refresh::*;
pub(crate) use tags::*;
pub(crate) use download::*;

pub trait Invertible {
  fn invert(&self) -> Self;
}
