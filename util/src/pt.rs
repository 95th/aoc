use crate::{dir::Dir, dist::Dist};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pt {
    pub i: i32,
    pub j: i32,
}

impl Pt {
    pub const fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }

    pub fn step(self, dir: Dir) -> Self {
        self + dir.into_distance()
    }
}

impl std::ops::Sub<Self> for Pt {
    type Output = Dist;

    fn sub(self, other: Self) -> Self::Output {
        Dist::new(self.i - other.i, self.j - other.j)
    }
}

impl std::ops::Add<Dist> for Pt {
    type Output = Self;

    fn add(self, dist: Dist) -> Self::Output {
        Self::new(self.i + dist.i, self.j + dist.j)
    }
}

impl std::ops::AddAssign<Dist> for Pt {
    fn add_assign(&mut self, dist: Dist) {
        self.i += dist.i;
        self.j += dist.j;
    }
}

impl std::ops::Sub<Dist> for Pt {
    type Output = Self;

    fn sub(self, dist: Dist) -> Self::Output {
        Self::new(self.i - dist.i, self.j - dist.j)
    }
}

impl std::ops::SubAssign<Dist> for Pt {
    fn sub_assign(&mut self, dist: Dist) {
        self.i -= dist.i;
        self.j -= dist.j;
    }
}
