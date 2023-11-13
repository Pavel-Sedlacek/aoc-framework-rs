use aoc_linear_algebra::vector::Vector;

pub struct BoundingBox {
    raw: [usize; 3],
    sorted: [usize; 3]
}

impl From<&Vector<usize, 3>> for BoundingBox {
    fn from(value: &Vector<usize, 3>) -> Self {
        let mut a = BoundingBox {
            raw: [*value.x(), *value.y(), *value.z()],
            sorted: [*value.x(), *value.y(), *value.z()]
        };
        a.sorted.sort();
        a
    }
}

impl BoundingBox {
    pub fn surface_area(&self) -> usize {
        2 * ((self.raw[0] * self.raw[1]) + (self.raw[0] * self.raw[2]) + (self.raw[1] * self.raw[2]))
    }

    pub fn surfaces(&self) -> [usize; 3] {
        [self.raw[0] * self.raw[1], self.raw[0] * self.raw[2], self.raw[1] * self.raw[2]]
    }

    pub fn volume(&self) -> usize {
        self.raw[0] * self.raw[1] * self.raw[2]
    }

    pub fn min_circumference(&self) -> usize {
        (self.sorted[0] + self.sorted[1]) * 2
    }
}
