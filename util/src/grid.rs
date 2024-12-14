use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::{dir::Dir, pos::Pos};

pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cols in self.data.iter() {
            for cell in cols.iter() {
                write!(f, "{:?} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cols in self.data.iter() {
            for cell in cols.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid<u8> {
    pub fn from_bytes(str: &str) -> Self {
        Self {
            data: str.lines().map(|line| line.as_bytes().to_vec()).collect(),
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
    pub fn new(width: usize, height: usize, value: T) -> Grid<T>
    where
        T: Clone,
    {
        Grid {
            data: vec![vec![value; width]; height],
        }
    }

    pub fn replace(&mut self, point: Pos, mut value: T) -> Option<T> {
        if self.contains_point(point) {
            std::mem::swap(&mut self[point], &mut value);
            Some(value)
        } else {
            None
        }
    }

    pub fn map<U: Clone>(&self, value: U) -> Grid<U> {
        Grid::new(self.cols(), self.rows(), value)
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data[0].len()
    }

    pub fn points(&self) -> impl Iterator<Item = Pos> {
        let rows = self.rows() as i32;
        let cols = self.cols() as i32;
        (0..rows).flat_map(move |y| (0..cols).map(move |x| Pos { x, y }))
    }

    pub fn get(&self, Pos { x, y }: Pos) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }

        self.data
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
    }

    pub fn contains_point(&self, Pos { x, y }: Pos) -> bool {
        x >= 0 && y >= 0 && y < self.rows() as i32 && x < self.cols() as i32
    }

    pub fn find(&self, filter: impl Fn(&T) -> bool) -> Option<Pos> {
        for (y, row) in self.data.iter().enumerate() {
            if let Some(x) = row.iter().position(&filter) {
                return Some(Pos::new(x as i32, y as i32));
            }
        }

        None
    }

    pub fn neighbors(&self, point: Pos) -> impl Iterator<Item = Pos> + '_ {
        Dir::all()
            .map(move |dir| point.step(dir))
            .filter(move |&p| self.contains_point(p))
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, Pos { x, y }: Pos) -> &Self::Output {
        assert!(x >= 0 && y >= 0);
        &self.data[y as usize][x as usize]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, Pos { x, y }: Pos) -> &mut Self::Output {
        assert!(x >= 0 && y >= 0);
        &mut self.data[y as usize][x as usize]
    }
}
