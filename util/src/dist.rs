#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dist {
    pub i: i32,
    pub j: i32,
}

impl Dist {
    pub const fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }
}

impl std::ops::Mul<i32> for Dist {
    type Output = Self;

    fn mul(self, value: i32) -> Self::Output {
        Self::new(self.i * value, self.j * value)
    }
}
