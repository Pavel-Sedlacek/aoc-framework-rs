pub struct Vector<T, const D: usize> {
    raw: [T; D]
}

impl <T, const D: usize> Vector<T, D> {
    pub fn new(components: [T; D]) -> Self {
        Vector {
            raw: components
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.raw.get(index)
    }
}

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
