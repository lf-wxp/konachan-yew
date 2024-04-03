use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::HtmlCanvasElement;
use yew::Event;

use crate::utils::{get_ctx, get_dpr, get_window};

use super::{intersect_2_lines, Flag, Line, Point, Timer};

pub(crate) struct PointLine {
  canvas: Option<HtmlCanvasElement>,
  line_num: i32,
  lines: Vec<Line>,
  line_color: String,
  dot_color: String,
  timer: Timer,
  this: Option<Rc<RefCell<Self>>>,
}

impl PointLine {
  pub fn new(
    canvas: Option<HtmlCanvasElement>,
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
    point_line
  }
  fn create_lines(&mut self) {
    if let Some(canvas) = &self.canvas {
      let width = canvas.width() as f32;
      let height = canvas.height() as f32;
      self.lines = (0..self.line_num)
        .map(|idx| {
          let flag = if idx % 2 == 0 { Flag::H } else { Flag::V };
          Line::new(flag, width, height)
        })
        .collect();
    }
  }
  pub fn resize(&mut self) -> Option<()> {
    if let Some(canvas) = &self.canvas {
      let parent_dom = canvas.parent_element()?;
      let dpr = get_dpr();
      let width = parent_dom.client_width() * dpr as i32;
      let height = parent_dom.client_height() * dpr as i32;
      canvas.set_width(width as u32);
      canvas.set_height(height as u32);
      self.create_lines();
      return Some(());
    }
    Some(())
  }
  pub fn set_color(&mut self, line_color: String, dot_color: String) {
    self.line_color = line_color;
    self.dot_color = dot_color;
  }
  pub fn set_canvas(&mut self, canvas: HtmlCanvasElement) {
    self.canvas = Some(canvas);
  }
  fn draw(&mut self) -> Result<(), JsValue> {
    if let Some(canvas) = &self.canvas {
      let ctx = get_ctx(&canvas)?;
      ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

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
      return Ok(());
    }
    Ok(())
  }
  pub fn bind_event(&self) {
    if let Some(pointline) = &self.this {
      let pointline = pointline.clone();
      let window = get_window();
      let closure = Closure::<dyn Fn(_)>::new(move |_: Event| {
        pointline.borrow_mut().resize();
      });
      window
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .ok();
      closure.forget();
    }
  }
  fn subscribe(&self) {
    if let Some(pointline) = &self.this {
      let pointline = pointline.clone();
      self.timer.subscribe(move || {
        let _ = pointline.borrow_mut().draw();
      });
    }
  }
  pub fn init(&mut self) {
    self.resize();
    self.bind_event();
    self.subscribe();
    self.timer.start();
  }
}
