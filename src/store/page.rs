use bounce::Slice;
use std::rc::Rc;
use yew::Reducible;

pub enum PageAction {
  Prev,
  Next,
  Total(usize),
  Invoke(usize),
}

#[derive(Slice, PartialEq, Clone, Debug)]
pub(crate) struct Page {
  pub current: usize,
  pub total: usize,
}

impl Page {
  pub fn new(current: usize, total: usize) -> Self {
    Page { current, total }
  }
}

impl Default for Page {
  fn default() -> Self {
    Self {
      current: 1,
      total: Default::default(),
    }
  }
}

impl Reducible for Page {
  type Action = PageAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      PageAction::Next => {
        let current = self.current + 1;
        if current <= self.total {
          return Page { current, ..(*self) }.into();
        }
        self
      }
      PageAction::Prev => {
        let current = self.current - 1;
        if current > 0 {
          return Page { current, ..(*self) }.into();
        }
        self
      }
      PageAction::Total(total) => Page { total, ..(*self) }.into(),
      PageAction::Invoke(current) => {
        if (1..self.total).contains(&current) {
          return Page { current, ..(*self) }.into();
        }
        self
      }
    }
  }
}
