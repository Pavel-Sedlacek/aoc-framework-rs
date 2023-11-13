/// Linear algebra Vector over T of size D
pub struct Vector<T, const R: usize> {
  raw: [T; R]
}

impl <T, const R: usize> Vector<T, R> {
  pub fn new(components: [T; R]) -> Self {
    Vector {
      raw: components
    }
  }

  pub fn get(&self, index: usize) -> Option<&T> {
    self.raw.get(index)
  }
}

/// Vector in LL^3 components
impl <T> Vector<T, 3> {
  pub fn x(&self) -> &T {
    &self.raw[0]
  }
  pub fn y(&self) -> &T {
    &self.raw[1]
  }
  pub fn z(&self) -> &T {
    &self.raw[2]
  }
}

/// Vector of vectors as a matrix
impl <T, const R: usize, const S: usize> Vector<Vector<T, R>, S> {
  
}