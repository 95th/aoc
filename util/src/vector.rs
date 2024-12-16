use crate::Dir;

pub const UP: Vec2 = vec2(0, -1);
pub const DOWN: Vec2 = vec2(0, 1);
pub const LEFT: Vec2 = vec2(-1, 0);
pub const RIGHT: Vec2 = vec2(1, 0);
pub const UP_LEFT: Vec2 = vec2(-1, -1);
pub const UP_RIGHT: Vec2 = vec2(1, -1);
pub const DOWN_LEFT: Vec2 = vec2(-1, 1);
pub const DOWN_RIGHT: Vec2 = vec2(1, 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

pub const fn vec2(x: i32, y: i32) -> Vec2 {
    Vec2 { x, y }
}

impl Vec2 {
    pub fn neighbor(self, dir: Dir) -> Self {
        self + dir.unit_vector()
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        vec2(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        vec2(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Add<i32> for Vec2 {
    type Output = Self;

    fn add(self, scalar: i32) -> Self {
        vec2(self.x + scalar, self.y + scalar)
    }
}

impl std::ops::AddAssign<i32> for Vec2 {
    fn add_assign(&mut self, scalar: i32) {
        self.x += scalar;
        self.y += scalar;
    }
}

impl std::ops::Sub<i32> for Vec2 {
    type Output = Self;

    fn sub(self, scalar: i32) -> Self {
        vec2(self.x - scalar, self.y - scalar)
    }
}

impl std::ops::SubAssign<i32> for Vec2 {
    fn sub_assign(&mut self, scalar: i32) {
        self.x -= scalar;
        self.y -= scalar;
    }
}

impl std::ops::Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        vec2(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::MulAssign<i32> for Vec2 {
    fn mul_assign(&mut self, scalar: i32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl std::ops::Div<i32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: i32) -> Self {
        vec2(self.x / scalar, self.y / scalar)
    }
}

impl std::ops::DivAssign<i32> for Vec2 {
    fn div_assign(&mut self, scalar: i32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}
