use crate::{dir::Dir, dist::Dist};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pt(pub i32, pub i32);

impl Pt {
    pub fn step(self, dir: Dir) -> Self {
        self + dir.into_distance()
    }
}

impl std::ops::Add<Dist> for Pt {
    type Output = Self;

    fn add(self, Dist(i, j): Dist) -> Self::Output {
        Self(self.0 + i, self.1 + j)
    }
}

impl std::ops::AddAssign<Dist> for Pt {
    fn add_assign(&mut self, Dist(i, j): Dist) {
        self.0 += i;
        self.1 += j;
    }
}

impl std::ops::Sub<Self> for Pt {
    type Output = Dist;

    fn sub(self, Self(i, j): Self) -> Self::Output {
        Dist(self.0 - i, self.1 - j)
    }
}

impl std::ops::Sub<Dist> for Pt {
    type Output = Self;

    fn sub(self, Dist(i, j): Dist) -> Self::Output {
        Self(self.0 - i, self.1 - j)
    }
}

impl std::ops::SubAssign<Dist> for Pt {
    fn sub_assign(&mut self, Dist(i, j): Dist) {
        self.0 -= i;
        self.1 -= j;
    }
}
