use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn step(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Point(self.0 - 1, self.1),
            Direction::Down => Point(self.0 + 1, self.1),
            Direction::Left => Point(self.0, self.1 - 1),
            Direction::Right => Point(self.0, self.1 + 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Distance(pub i32, pub i32);

impl std::ops::Mul<i32> for Distance {
    type Output = Distance;

    fn mul(self, value: i32) -> Self::Output {
        Distance(self.0 * value, self.1 * value)
    }
}

impl std::ops::Add<Distance> for Point {
    type Output = Point;

    fn add(self, Distance(i, j): Distance) -> Self::Output {
        Point(self.0 + i, self.1 + j)
    }
}

impl std::ops::AddAssign<Distance> for Point {
    fn add_assign(&mut self, Distance(i, j): Distance) {
        self.0 += i;
        self.1 += j;
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Distance;

    fn sub(self, Point(i, j): Point) -> Self::Output {
        Distance(self.0 - i, self.1 - j)
    }
}

impl std::ops::Sub<Distance> for Point {
    type Output = Point;

    fn sub(self, Distance(i, j): Distance) -> Self::Output {
        Point(self.0 - i, self.1 - j)
    }
}

impl std::ops::SubAssign<Distance> for Point {
    fn sub_assign(&mut self, Distance(i, j): Distance) {
        self.0 -= i;
        self.1 -= j;
    }
}

pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
}

impl Matrix<u8> {
    pub fn from_bytes(str: &str) -> Self {
        Self {
            data: str.lines().map(|line| line.bytes().collect()).collect(),
        }
    }
}

impl<T: FromStr> Matrix<T> {
    pub fn parse(str: &str) -> Result<Self, T::Err> {
        Ok(Self {
            data: str
                .lines()
                .map(|line| line.split_whitespace().map(|s| s.parse()).collect())
                .collect::<Result<Vec<Vec<_>>, _>>()?,
        })
    }
}

impl<T> Matrix<T> {
    pub fn replace(&mut self, point: Point, mut value: T) -> Option<T> {
        if self.has_point(point) {
            std::mem::swap(&mut self[point], &mut value);
            Some(value)
        } else {
            None
        }
    }

    pub fn map<U: Clone>(&self, value: U) -> Matrix<U> {
        Matrix {
            data: vec![vec![value; self.cols()]; self.rows()],
        }
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data[0].len()
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point> {
        let rows = self.rows() as i32;
        let cols = self.cols() as i32;
        (0..rows).flat_map(move |i| (0..cols).map(move |j| Point(i, j)))
    }

    pub fn get(&self, Point(i, j): Point) -> Option<&T> {
        if i < 0 || j < 0 {
            return None;
        }

        self.data
            .get(i as usize)
            .and_then(|row| row.get(j as usize))
    }

    pub fn has_point(&self, Point(i, j): Point) -> bool {
        i >= 0 && j >= 0 && i < self.rows() as i32 && j < self.cols() as i32
    }

    pub fn find(&self, filter: impl Fn(&T) -> bool) -> Option<Point> {
        for (i, row) in self.data.iter().enumerate() {
            if let Some(j) = row.iter().position(|cell| filter(cell)) {
                return Some(Point(i as i32, j as i32));
            }
        }

        None
    }
}

impl<T> Index<Point> for Matrix<T> {
    type Output = T;

    fn index(&self, Point(i, j): Point) -> &Self::Output {
        assert!(i >= 0 && j >= 0);
        &self.data[i as usize][j as usize]
    }
}

impl<T> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, Point(i, j): Point) -> &mut Self::Output {
        assert!(i >= 0 && j >= 0);
        &mut self.data[i as usize][j as usize]
    }
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}
