#[derive(Debug)]
pub(crate) struct Point {
  pub x: f32,
  pub y: f32,
}

impl Point {
  pub fn new(x: f32, y: f32) -> Self {
    Point { x, y }
  }
}
