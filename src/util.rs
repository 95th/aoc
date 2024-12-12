use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Down, Self::Left, Self::Right].into_iter()
    }

    pub fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn into_distance(self) -> Dist {
        match self {
            Self::Up => Dist(-1, 0),
            Self::Down => Dist(1, 0),
            Self::Left => Dist(0, -1),
            Self::Right => Dist(0, 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pt(pub i32, pub i32);

impl Pt {
    pub fn step(self, dir: Dir) -> Self {
        self + dir.into_distance()
    }
}

impl std::fmt::Debug for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dist(pub i32, pub i32);

impl std::ops::Mul<i32> for Dist {
    type Output = Dist;

    fn mul(self, value: i32) -> Self::Output {
        Dist(self.0 * value, self.1 * value)
    }
}

impl std::ops::Add<Dist> for Pt {
    type Output = Pt;

    fn add(self, Dist(i, j): Dist) -> Self::Output {
        Pt(self.0 + i, self.1 + j)
    }
}

impl std::ops::AddAssign<Dist> for Pt {
    fn add_assign(&mut self, Dist(i, j): Dist) {
        self.0 += i;
        self.1 += j;
    }
}

impl std::ops::Sub<Pt> for Pt {
    type Output = Dist;

    fn sub(self, Pt(i, j): Pt) -> Self::Output {
        Dist(self.0 - i, self.1 - j)
    }
}

impl std::ops::Sub<Dist> for Pt {
    type Output = Pt;

    fn sub(self, Dist(i, j): Dist) -> Self::Output {
        Pt(self.0 - i, self.1 - j)
    }
}

impl std::ops::SubAssign<Dist> for Pt {
    fn sub_assign(&mut self, Dist(i, j): Dist) {
        self.0 -= i;
        self.1 -= j;
    }
}

pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

impl Grid<u8> {
    pub fn from_bytes(str: &str) -> Self {
        Self {
            data: str.lines().map(|line| line.bytes().collect()).collect(),
        }
    }
}

impl<T: FromStr> Grid<T> {
    pub fn parse(str: &str) -> Result<Self, T::Err> {
        Ok(Self {
            data: str
                .lines()
                .map(|line| line.split_whitespace().map(|s| s.parse()).collect())
                .collect::<Result<Vec<Vec<_>>, _>>()?,
        })
    }
}

impl<T> Grid<T> {
    pub fn replace(&mut self, point: Pt, mut value: T) -> Option<T> {
        if self.contains_point(point) {
            std::mem::swap(&mut self[point], &mut value);
            Some(value)
        } else {
            None
        }
    }

    pub fn map<U: Clone>(&self, value: U) -> Grid<U> {
        Grid {
            data: vec![vec![value; self.cols()]; self.rows()],
        }
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data[0].len()
    }

    pub fn points(&self) -> impl Iterator<Item = Pt> {
        let rows = self.rows() as i32;
        let cols = self.cols() as i32;
        (0..rows).flat_map(move |i| (0..cols).map(move |j| Pt(i, j)))
    }

    pub fn get(&self, Pt(i, j): Pt) -> Option<&T> {
        if i < 0 || j < 0 {
            return None;
        }

        self.data
            .get(i as usize)
            .and_then(|row| row.get(j as usize))
    }

    pub fn contains_point(&self, Pt(i, j): Pt) -> bool {
        i >= 0 && j >= 0 && i < self.rows() as i32 && j < self.cols() as i32
    }

    pub fn find(&self, filter: impl Fn(&T) -> bool) -> Option<Pt> {
        for (i, row) in self.data.iter().enumerate() {
            if let Some(j) = row.iter().position(&filter) {
                return Some(Pt(i as i32, j as i32));
            }
        }

        None
    }

    pub fn neighbors(&self, point: Pt) -> impl Iterator<Item = Pt> + '_ {
        Dir::all()
            .map(move |dir| point.step(dir))
            .filter(move |&p| self.contains_point(p))
    }
}

impl<T> Index<Pt> for Grid<T> {
    type Output = T;

    fn index(&self, Pt(i, j): Pt) -> &Self::Output {
        assert!(i >= 0 && j >= 0);
        &self.data[i as usize][j as usize]
    }
}

impl<T> IndexMut<Pt> for Grid<T> {
    fn index_mut(&mut self, Pt(i, j): Pt) -> &mut Self::Output {
        assert!(i >= 0 && j >= 0);
        &mut self.data[i as usize][j as usize]
    }
}
