#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dist(pub i32, pub i32);

impl std::ops::Mul<i32> for Dist {
    type Output = Self;

    fn mul(self, value: i32) -> Self::Output {
        Self(self.0 * value, self.1 * value)
    }
}
