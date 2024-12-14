#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dist {
    pub x: i32,
    pub y: i32,
}

impl Dist {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Mul<i32> for Dist {
    type Output = Self;

    fn mul(self, value: i32) -> Self::Output {
        Self::new(self.x * value, self.y * value)
    }
}
