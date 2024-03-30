use super::Point;
use gloo_console::log;
use rand::Rng;

fn random_int_from_interval(mn: f32, mx: f32) -> f32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(mn..=mx) as f32
}
pub fn intersect_2_lines(l1: &Line, l2: &Line) -> Option<Point> {
  let p1 = &l1.a;
  let p2 = &l1.b;
  let p3 = &l2.a;
  let p4 = &l2.b;

  let denominator = (p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y);
  if denominator == 0.0 {
    return None;
  }
  let ua = ((p4.x - p3.x) * (p1.y - p3.y) - (p4.y - p3.y) * (p1.x - p3.x)) / denominator;
  let ub = ((p2.x - p1.x) * (p1.y - p3.y) - (p2.y - p1.y) * (p1.x - p3.x)) / denominator;

  if ua > 0.0 && ub > 0.0 {
    Some(Point {
      x: p1.x + ua * (p2.x - p1.x),
      y: p1.y + ua * (p2.y - p1.y),
    })
  } else {
    None
  }
}
#[derive(Debug)]
pub(crate) enum Flag {
  H,
  V,
}

#[derive(Debug)]
pub(crate) struct Line {
  flag: Flag,
  va: f32,
  vb: f32,
  w: f32,
  h: f32,
  pub a: Point,
  pub b: Point,
}

impl Line {
  pub fn new(flag: Flag, w: f32, h: f32) -> Self {
    let mut line = Line {
      flag,
      va: 0.0,
      vb: 0.0,
      w,
      h,
      a: Point::new(0.0, 0.0),
      b: Point::new(0.0, 0.0),
    };
    line.init();
    line
  }
  fn init(&mut self) {
    match self.flag {
      Flag::H => {
        self.a.x = 0.0;
        self.a.y = random_int_from_interval(0.0, self.w);
        self.b.x = self.w;
        self.b.y = random_int_from_interval(0.0, self.w);
      }
      Flag::V => {
        self.a.y = 0.0;
        self.a.x = random_int_from_interval(0.0, self.h);
        self.b.y = self.h;
        self.b.x = random_int_from_interval(0.0, self.h);
      }
    }
    self.va = random_int_from_interval(25.0, 100.0) / 100.0;
    self.vb = random_int_from_interval(25.0, 100.0) / 100.0;
    log!("init", self.va, self.vb);
  }

  fn edges(&mut self) {
    match self.flag {
      Flag::H => {
        if self.a.y < 0.0 || self.a.y > self.h {
          self.va *= -1.0;
        }
        if self.b.y < 0.0 || self.b.y > self.h {
          self.vb *= -1.0;
        }
      }
      Flag::V => {
        if self.a.x < 0.0 || self.a.x > self.w {
          self.va *= -1.0;
        }
        if self.b.x < 0.0 || self.b.x > self.w {
          self.vb *= -1.0;
        }
      }
    }
  }

  pub fn update(&mut self) {
    match self.flag {
      Flag::H => {
        self.a.y += self.va;
        self.b.y += self.vb;
      }
      Flag::V => {
        self.a.x += self.va;
        self.b.x += self.vb;
      }
    };
    self.edges();
  }
}
