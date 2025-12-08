use crate::Dir;

pub const UP: Vec2 = vec2(0, -1);
pub const DOWN: Vec2 = vec2(0, 1);
pub const LEFT: Vec2 = vec2(-1, 0);
pub const RIGHT: Vec2 = vec2(1, 0);
pub const UP_LEFT: Vec2 = vec2(-1, -1);
pub const UP_RIGHT: Vec2 = vec2(1, -1);
pub const DOWN_LEFT: Vec2 = vec2(-1, 1);
pub const DOWN_RIGHT: Vec2 = vec2(1, 1);
pub const ZERO: Vec2 = vec2(0, 0);

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

    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        Dir::all().map(move |dir| self.neighbor(dir))
    }

    pub fn neighbors_all(self) -> impl Iterator<Item = Self> {
        [
            UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
        ]
        .into_iter()
        .map(move |dir| self + dir)
    }

    pub fn manhattan_dist(self, other: Vec2) -> i32 {
        (self - other).manhattan()
    }

    pub fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

pub const fn vec3(x: isize, y: isize, z: isize) -> Vec3 {
    Vec3 { x, y, z }
}

impl Vec3 {
    pub fn parse_csv(input: &str) -> Self {
        let mut list = [0, 0, 0];
        for (i, x) in input.split(',').map(|x| x.parse().unwrap()).enumerate() {
            list[i] = x;
        }
        Self {
            x: list[0],
            y: list[1],
            z: list[2],
        }
    }

    pub fn euclidean_dist(self, other: Vec3) -> isize {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}
