/// Algebraic division ring defining two binary operations and the inverse of an element
pub trait DivisionRing<T> {
  fn add(&self, other: T) -> T;
  fn times(&self, other: T) -> T;
  fn inverse(&self) -> T;
}