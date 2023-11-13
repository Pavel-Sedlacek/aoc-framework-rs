use super::algebra::division_ring::DivisionRing;

// A generic bounding box for objects in space T^D
pub struct BoundingBox<T: DivisionRing<T>, const D: usize> {
  raw: [T; D]
}

impl <T: DivisionRing<T>, const D: usize> BoundingBox<T, D> {
}
