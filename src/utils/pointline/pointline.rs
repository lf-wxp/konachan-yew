use std::cell::RefCell;
use std::rc::Rc;

use gloo_console::log;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::utils::get_dpr;

use super::{intersect_2_lines, Flag, Line, Point, Timer};

pub(crate) struct PointLine {
  canvas: HtmlCanvasElement,
  line_num: i32,
  lines: Vec<Line>,
  line_color: String,
  dot_color: String,
  timer: Timer,
  this: Option<Rc<RefCell<Self>>>,
}

impl PointLine {
  pub fn new(
    canvas: HtmlCanvasElement,
    line_num: i32,
    line_color: String,
    dot_color: String,
  ) -> Rc<RefCell<Self>> {
    let point_line = Rc::new(RefCell::new(PointLine {
      canvas,
      line_num,
      lines: vec![],
      line_color,
      dot_color,
      timer: Timer::new(),
      this: None,
    }));
    point_line.borrow_mut().this = Some(point_line.clone());
    point_line.borrow_mut().resize();
    point_line.borrow().init();
    point_line
  }
  fn get_ctx(&self) -> Result<CanvasRenderingContext2d, JsValue> {
    let ctx = self
      .canvas
      .get_context("2d")?
      .ok_or("")?
      .dyn_into::<CanvasRenderingContext2d>()
      .ok()
      .ok_or("")?;
    Ok(ctx)
  }
  fn create_lines(&mut self) {
    let width = self.canvas.width() as f32;
    let height = self.canvas.height() as f32;
    self.lines = (0..self.line_num)
      .map(|idx| {
        let flag = if idx % 2 == 0 { Flag::H } else { Flag::V };
        Line::new(flag, width, height)
      })
      .collect();
  }
  pub fn resize(&mut self) -> Option<()>{
    let parent_dom = self.canvas.parent_element()?;
    let dpr = get_dpr();
    let width = parent_dom.client_width() * dpr as i32;
    let height = parent_dom.client_height() * dpr as i32;
    self.canvas.set_width(width as u32);
    self.canvas.set_height(height as u32);
    self.create_lines();
    Some(()) 
  }
  pub fn set_color(&mut self, line_color: Option<String>, dot_color: Option<String>) {
    if let Some(color) = line_color {
      self.line_color = color;
    }
    if let Some(color) = dot_color {
      self.dot_color = color;
    }
  }
  fn draw(&mut self) -> Result<(), JsValue> {
    let ctx = self.get_ctx()?;
    ctx.clear_rect(
      0.0,
      0.0,
      self.canvas.width().into(),
      self.canvas.height().into(),
    );

    for line in &mut self.lines {
      ctx.set_stroke_style(&self.line_color.clone().into());
      ctx.begin_path();
      ctx.move_to(line.a.x as f64, line.a.y as f64);
      ctx.line_to(line.b.x as f64, line.b.y as f64);
      ctx.stroke();
      line.update();
    }

    let mut points: Vec<Point> = vec![];
    for (idx, line) in self.lines.iter().enumerate() {
      for l in self.lines.iter().skip(idx + 1) {
        if let Some(point) = intersect_2_lines(line, l) {
          points.push(point);
        }
      }
    }

    for point in points {
      ctx.begin_path();
      let _ = ctx.arc(
        point.x.into(),
        point.y.into(),
        2.0,
        0.0,
        2.0 * std::f64::consts::PI,
      );
      ctx.set_fill_style(&self.dot_color.clone().into());
      ctx.fill();
    }
    Ok(())
  }
  fn subscribe(&self) {
    if let Some(ribbons) = &self.this {
      let ribbons = ribbons.clone();
      self.timer.subscribe(move || {
        let _ = ribbons.borrow_mut().draw();
      });
    }
  }
  pub fn init(&self) {
    self.subscribe();
    self.timer.start();
  }
}
