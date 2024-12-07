use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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

pub struct Matrix {
    pub data: Vec<Vec<u8>>,
}

impl Matrix {
    pub fn parse(str: &str) -> Self {
        Self {
            data: str.lines().map(|line| line.bytes().collect()).collect(),
        }
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point> {
        let rows = self.data.len() as i32;
        let cols = self.data[0].len() as i32;
        (0..rows).flat_map(move |i| (0..cols).map(move |j| Point(i, j)))
    }

    pub fn get(&self, Point(i, j): Point) -> Option<u8> {
        if i < 0 || j < 0 {
            return None;
        }

        self.data
            .get(i as usize)
            .and_then(|row| row.get(j as usize))
            .copied()
    }

    pub fn find(&self, filter: impl Fn(u8) -> bool) -> Option<Point> {
        for (i, row) in self.data.iter().enumerate() {
            if let Some(j) = row.iter().position(|&cell| filter(cell)) {
                return Some(Point(i as i32, j as i32));
            }
        }

        None
    }
}

impl Index<Point> for Matrix {
    type Output = u8;

    fn index(&self, Point(i, j): Point) -> &Self::Output {
        assert!(i >= 0 && j >= 0);
        &self.data[i as usize][j as usize]
    }
}

impl IndexMut<Point> for Matrix {
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
