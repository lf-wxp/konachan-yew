use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, HtmlImageElement};

use crate::utils::{get_ctx, get_dpr, Timer};

const ROW: u8 = 9;
const COLUMN: u8 = 16;
const LEN: u8 = ROW * COLUMN;
const RATIO: f64 = 16.0 / 9.0;

#[derive(Debug)]
pub struct Section {
  pub x: f64,
  pub y: f64,
}
pub(crate) struct ParticleProgress {
  canvas: Option<HtmlCanvasElement>,
  color: String,
  width: f64,
  height: f64,
  timer: Timer,
  section: Vec<Section>,
  image: Option<HtmlImageElement>,
  ratio: f64,
  percent: f64,
  alpha: Cell<f64>,
  alpha_step: Cell<f64>,
  this: Option<Rc<RefCell<Self>>>,
}

impl ParticleProgress {
  pub fn new(
    canvas: Option<HtmlCanvasElement>,
    image: Option<HtmlImageElement>,
    color: String,
  ) -> Rc<RefCell<Self>> {
    let point_line = Rc::new(RefCell::new(ParticleProgress {
      canvas,
      color,
      width: 0.0,
      height: 0.0,
      image,
      section: vec![],
      ratio: 1.0,
      percent: 0.0,
      alpha: Cell::new(0.0),
      alpha_step: Cell::new(0.02),
      timer: Timer::new(),
      this: None,
    }));
    point_line.borrow_mut().this = Some(point_line.clone());
    point_line
  }

  fn calc_ratio(&mut self) {
    if let Some(image) = &self.image {
      let natural_width = image.natural_width() as f64;
      let natural_height = image.natural_height() as f64;
      let width = RATIO * natural_height;
      self.ratio = {
        if width <= natural_width {
          natural_height / self.height
        } else {
          natural_width / self.width
        }
      }
    }
  }

  fn create_section(&mut self) {
    let size = self.width / COLUMN as f64;
    for i in 0..ROW {
      for j in 0..COLUMN {
        self.section.push(Section {
          x: j as f64 * size,
          y: i as f64 * size,
        });
      }
    }
  }
  pub fn set_color(&mut self, color: String) {
    self.color = color;
  }
  pub fn set_canvas(&mut self, canvas: HtmlCanvasElement) -> Option<()> {
    let parent = canvas.parent_element()?;
    let width = parent.client_width() as u32;
    let height = parent.client_height() as u32;
    let dpr = get_dpr() as u32;
    canvas.set_width(width * dpr);
    canvas.set_height(height * dpr);
    self.width = width as f64;
    self.height = height as f64;
    self.canvas = Some(canvas);
    Some(())
  }
  pub fn set_percent(&mut self, percent: f64) {
    self.percent = percent;
  }
  pub fn set_image(&mut self, image: HtmlImageElement) {
    self.image = Some(image);
  }
  fn change_alpha(&self) {
    let mut alpha = self.alpha.get();
    let step = self.alpha_step.get();
    alpha += step;
    if alpha > 1.0 {
      self.alpha_step.set(-0.02);
    }
    if alpha < 0.0 {
      self.alpha_step.set(0.02);
    }
    if (0.0..=1.0).contains(&alpha) {
      self.alpha.set(alpha);
    }
  }

  fn draw_particle(&mut self) -> Result<(), JsValue> {
    if self.percent < 100.0 {
      let canvas = self.canvas.as_ref().ok_or("get canvas error")?;
      let image = self.image.as_ref().ok_or("get image error")?;
      let ctx = get_ctx(canvas)?;
      let size = self.width / COLUMN as f64;
      let per_sect_percent = 100.0 / LEN as f64;
      let current = (self.percent / per_sect_percent).ceil();
      let dpr = get_dpr();
      ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
      for (idx, section) in self.section.iter().enumerate() {
        let idx = idx as f64;
        let Section { x, y } = *section;
        if idx < current {
          ctx.set_global_alpha(1.0);
          let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            image,
            self.ratio * x,
            self.ratio * y,
            self.ratio * size,
            self.ratio * size,
            x * dpr,
            y * dpr,
            size * dpr,
            size * dpr,
          );
        }
        if idx == current {
          let alpha = self.alpha.get();
          ctx.set_fill_style(&JsValue::from_str(&self.color));
          ctx.set_global_alpha(alpha);
          self.change_alpha();
          ctx.fill_rect(x * dpr, y * dpr, size * dpr, size * dpr);
        }
      }
    }
    Ok(())
  }

  fn draw_full(&mut self) -> Result<(), JsValue> {
    if self.percent >= 100.0 {
      let canvas = self.canvas.as_ref().ok_or("get canvas error")?;
      let image = self.image.as_ref().ok_or("get image error")?;
      let dpr = get_dpr();
      let ctx = get_ctx(canvas)?;
      ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
      ctx.set_global_alpha(1.0);
      let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        image,
        0.0,
        0.0,
        self.width * self.ratio,
        self.height * self.ratio,
        0.0,
        0.0,
        self.width * dpr,
        self.height * dpr,
      );
      self.timer.stop();
    };
    Ok(())
  }

  fn draw(&mut self) -> Result<(), JsValue> {
    self.draw_particle().unwrap();
    self.draw_full().unwrap();
    Ok(())
  }
  fn subscribe(&self) {
    if let Some(progress) = &self.this {
      let progress = progress.clone();
      self.timer.subscribe(move || {
        let _ = progress.borrow_mut().draw();
      });
    }
  }
  pub fn init(&mut self) {
    self.calc_ratio();
    self.create_section();
    self.subscribe();
  }

  pub fn start(&mut self) {
    self.timer.start();
    self.set_percent(0.0);
  }

  pub fn stop(&self) {
    self.timer.stop();
  }
}
