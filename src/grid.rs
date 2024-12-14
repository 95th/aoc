use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::{dir::Dir, pt::Pt};

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

    pub fn replace(&mut self, point: Pt, mut value: T) -> Option<T> {
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

    pub fn points(&self) -> impl Iterator<Item = Pt> {
        let rows = self.rows() as i32;
        let cols = self.cols() as i32;
        (0..rows).flat_map(move |i| (0..cols).map(move |j| Pt { i, j }))
    }

    pub fn get(&self, Pt { i, j }: Pt) -> Option<&T> {
        if i < 0 || j < 0 {
            return None;
        }

        self.data
            .get(i as usize)
            .and_then(|row| row.get(j as usize))
    }

    pub fn contains_point(&self, Pt { i, j }: Pt) -> bool {
        i >= 0 && j >= 0 && i < self.rows() as i32 && j < self.cols() as i32
    }

    pub fn find(&self, filter: impl Fn(&T) -> bool) -> Option<Pt> {
        for (i, row) in self.data.iter().enumerate() {
            if let Some(j) = row.iter().position(&filter) {
                return Some(Pt::new(i as i32, j as i32));
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

    fn index(&self, Pt { i, j }: Pt) -> &Self::Output {
        assert!(i >= 0 && j >= 0);
        &self.data[i as usize][j as usize]
    }
}

impl<T> IndexMut<Pt> for Grid<T> {
    fn index_mut(&mut self, Pt { i, j }: Pt) -> &mut Self::Output {
        assert!(i >= 0 && j >= 0);
        &mut self.data[i as usize][j as usize]
    }
}
