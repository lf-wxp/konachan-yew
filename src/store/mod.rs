pub mod download;
pub mod image;
pub mod loading;
pub mod mode;
pub mod page;
pub mod refresh;
pub mod security;
pub mod size;
pub mod state;
pub mod tags;
pub mod theme;

pub(crate) use download::*;
pub(crate) use image::*;
pub(crate) use loading::*;
pub(crate) use mode::*;
pub(crate) use page::*;
pub(crate) use refresh::*;
pub(crate) use security::*;
pub(crate) use size::*;
pub(crate) use state::*;
pub(crate) use tags::*;
pub(crate) use theme::*;

pub trait Invertible {
  fn invert(&self) -> Self;
}
