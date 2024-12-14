use crate::{dir::Dir, dist::Dist};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn step(self, dir: Dir) -> Self {
        self + dir.unit_vector()
    }
}

impl std::ops::Sub<Self> for Pos {
    type Output = Dist;

    fn sub(self, other: Self) -> Self::Output {
        Dist::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Add<Dist> for Pos {
    type Output = Self;

    fn add(self, dist: Dist) -> Self::Output {
        Self::new(self.x + dist.x, self.y + dist.y)
    }
}

impl std::ops::AddAssign<Dist> for Pos {
    fn add_assign(&mut self, dist: Dist) {
        self.x += dist.x;
        self.y += dist.y;
    }
}

impl std::ops::Sub<Dist> for Pos {
    type Output = Self;

    fn sub(self, dist: Dist) -> Self::Output {
        Self::new(self.x - dist.x, self.y - dist.y)
    }
}

impl std::ops::SubAssign<Dist> for Pos {
    fn sub_assign(&mut self, dist: Dist) {
        self.x -= dist.x;
        self.y -= dist.y;
    }
}
