/// Trait for closures acting on a generic type
pub trait ActingClosures<T, U> {
  /// returns a new value returned by the closure 
  fn apply<F>(self, closure: F) -> U where F: Fn(T) -> U;
  /// returns the value on which the closure is applied
  fn also<F>(self, closure: F) -> T where F: Fn(&T);
}

impl <T, U> ActingClosures<T, U> for T {
  fn apply<F>(self, closure: F) -> U where F: Fn(T) -> U {
    closure(self)
  }

  fn also<F>(self, closure: F) -> T where F: Fn(&T) {
    closure(&self);
    self
  }
}