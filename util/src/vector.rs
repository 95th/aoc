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
    pub x: isize,
    pub y: isize,
}

pub const fn vec2(x: isize, y: isize) -> Vec2 {
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

    pub fn manhattan_dist(self, other: Vec2) -> isize {
        (self - other).manhattan()
    }

    pub fn manhattan(&self) -> isize {
        self.x.abs() + self.y.abs()
    }

    /// Iterate over the points in the rectangle defined by the two given diagonal opposite points.
    pub fn rect_points(a: Self, b: Self) -> impl Iterator<Item = Vec2> {
        let (x1, x2) = (a.x.min(b.x), a.x.max(b.x));
        let (y1, y2) = (a.y.min(b.y), a.y.max(b.y));
        (y1..=y2).flat_map(move |y| (x1..=x2).map(move |x| vec2(x, y)))
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

impl std::ops::Add<isize> for Vec2 {
    type Output = Self;

    fn add(self, scalar: isize) -> Self {
        vec2(self.x + scalar, self.y + scalar)
    }
}

impl std::ops::AddAssign<isize> for Vec2 {
    fn add_assign(&mut self, scalar: isize) {
        self.x += scalar;
        self.y += scalar;
    }
}

impl std::ops::Sub<isize> for Vec2 {
    type Output = Self;

    fn sub(self, scalar: isize) -> Self {
        vec2(self.x - scalar, self.y - scalar)
    }
}

impl std::ops::SubAssign<isize> for Vec2 {
    fn sub_assign(&mut self, scalar: isize) {
        self.x -= scalar;
        self.y -= scalar;
    }
}

impl std::ops::Mul<isize> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: isize) -> Self {
        vec2(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::MulAssign<isize> for Vec2 {
    fn mul_assign(&mut self, scalar: isize) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl std::ops::Div<isize> for Vec2 {
    type Output = Self;

    fn div(self, scalar: isize) -> Self {
        vec2(self.x / scalar, self.y / scalar)
    }
}

impl std::ops::DivAssign<isize> for Vec2 {
    fn div_assign(&mut self, scalar: isize) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

pub fn range(start: Vec2, end: Vec2) -> impl Iterator<Item = Vec2> {
    if start.x != end.x && start.y != end.y {
        panic!("Range must be horizontal or vertical");
    }
    let step = vec2((end.x - start.x).signum(), (end.y - start.y).signum());
    let mut current = start;
    std::iter::from_fn(move || {
        if current == end {
            return None;
        }
        let next = current + step;
        current = next;
        Some(current)
    })
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
