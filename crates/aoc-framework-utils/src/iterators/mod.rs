use std::rc::Rc;

/// A collection of views into a generic value that covers all possible continuos sizes
pub trait GrowingWindow<T> {
  /// Generates all views
  fn growing_window(&self) -> Vec<Rc<&[T]>>;
}

impl <T> GrowingWindow<T> for Vec<T> {
  fn growing_window(&self) -> Vec<Rc<&[T]>> {
    let mut windows = vec![];
    for i in 1..self.len() {
      windows.push(Rc::new(&self[0..i]));
    }
    return windows
  }
}

pub trait ListSelectors<T> {
  fn first_that<F>(&self, predicate: F) -> Option<(&T, usize)> where F: Fn(&T) -> bool;
  fn last_that<F>(&self, predicate: F) -> Option<(&T, usize)> where F: Fn(&T) -> bool;
}

impl <T> ListSelectors<T> for Vec<T> {
  fn first_that<F>(&self, predicate: F) -> Option<(&T, usize)> where F: Fn(&T) -> bool {
    for i in 0..self.len() {
      if predicate(&self[i]) {
        return Some((&self[i], i))
      }
    }
    None
  }

  fn last_that<F>(&self, predicate: F) -> Option<(&T, usize)> where F: Fn(&T) -> bool {
    for i in self.len()..0 {
      if predicate(&self[i]) {
        return Some((&self[i], i))
      }
    }
    None
  }
}

pub trait CharsToString {
  fn to_string(&self) -> String;
}

impl CharsToString for [char] {
  fn to_string(&self) -> String {
    self.iter().collect()
  }
}