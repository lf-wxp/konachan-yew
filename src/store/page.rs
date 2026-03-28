use super::Slice;
use std::rc::Rc;

/// Actions that can be dispatched to the Page reducer.
pub enum PageAction {
  Prev,
  Next,
  Total(usize),
  Invoke(usize),
}

/// Pagination state holding current page and total page count.
#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Page {
  pub current: usize,
  pub total: usize,
}

impl Slice for Page {
  type Action = PageAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      PageAction::Next => {
        if self.current < self.total {
          return Rc::new(Page {
            current: self.current + 1,
            ..(*self)
          });
        }
        self
      }
      PageAction::Prev => {
        if self.current > 1 {
          return Rc::new(Page {
            current: self.current - 1,
            ..(*self)
          });
        }
        self
      }
      PageAction::Total(total) => Rc::new(Page { total, ..(*self) }),
      PageAction::Invoke(current) => {
        if (1..=self.total).contains(&current) {
          return Rc::new(Page { current, ..(*self) });
        }
        self
      }
    }
  }
}

impl Page {
  #[allow(dead_code)]
  pub fn new(current: usize, total: usize) -> Self {
    Page { current, total }
  }
}

impl Default for Page {
  fn default() -> Self {
    Self {
      current: 1,
      total: 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn page(current: usize, total: usize) -> Rc<Page> {
    Rc::new(Page::new(current, total))
  }

  #[test]
  fn test_default_page() {
    let p = Page::default();
    assert_eq!(p.current, 1);
    assert_eq!(p.total, 0);
  }

  #[test]
  fn test_next_increments() {
    let p = page(1, 10);
    let p = p.reduce(PageAction::Next);
    assert_eq!(p.current, 2);
  }

  #[test]
  fn test_next_at_last_page_stays() {
    let p = page(10, 10);
    let p = p.reduce(PageAction::Next);
    assert_eq!(p.current, 10, "should not exceed total");
  }

  #[test]
  fn test_prev_decrements() {
    let p = page(5, 10);
    let p = p.reduce(PageAction::Prev);
    assert_eq!(p.current, 4);
  }

  #[test]
  fn test_prev_at_first_page_stays() {
    let p = page(1, 10);
    let p = p.reduce(PageAction::Prev);
    assert_eq!(p.current, 1, "should not go below 1");
  }

  #[test]
  fn test_total_updates() {
    let p = page(1, 0);
    let p = p.reduce(PageAction::Total(100));
    assert_eq!(p.total, 100);
    assert_eq!(p.current, 1, "current should not change");
  }

  #[test]
  fn test_invoke_valid_page() {
    let p = page(1, 10);
    let p = p.reduce(PageAction::Invoke(5));
    assert_eq!(p.current, 5);
  }

  #[test]
  fn test_invoke_out_of_range_stays() {
    let p = page(1, 10);
    let p = p.reduce(PageAction::Invoke(0));
    assert_eq!(p.current, 1, "page 0 is invalid");

    let p = page(1, 10);
    let p = p.reduce(PageAction::Invoke(11));
    assert_eq!(p.current, 1, "page 11 exceeds total");
  }

  #[test]
  fn test_invoke_boundary_pages() {
    let p = page(5, 10);
    let p = p.reduce(PageAction::Invoke(1));
    assert_eq!(p.current, 1, "first page should be valid");

    let p = page(5, 10);
    let p = p.reduce(PageAction::Invoke(10));
    assert_eq!(p.current, 10, "last page should be valid");
  }
}
