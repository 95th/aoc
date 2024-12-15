use std::{
    mem,
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::{Dir, Vec2};

pub struct Grid<T> {
    data: Vec<T>,
    cols: usize,
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.chunks_exact(self.cols) {
            for cell in row {
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
        for row in self.data.chunks_exact(self.cols) {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid<u8> {
    pub fn from_bytes(str: &str) -> Self {
        let mut data = vec![];
        let mut cols = 0;
        for line in str.lines() {
            data.extend_from_slice(line.as_bytes());
            if cols == 0 {
                cols = line.len();
            }
        }
        assert!(cols > 0);
        assert_eq!(data.len() % cols, 0);
        Self { data, cols }
    }
}

impl<T: FromStr> Grid<T> {
    pub fn parse(str: &str) -> Result<Self, T::Err> {
        let mut data = vec![];
        let mut cols = 0;

        for line in str.lines() {
            for value in line.split_whitespace() {
                data.push(value.parse()?);
            }
            if cols == 0 {
                cols = data.len();
            }
        }

        assert!(cols > 0);
        assert_eq!(data.len() % cols, 0);
        Ok(Self { data, cols })
    }
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, value: T) -> Grid<T>
    where
        T: Clone,
    {
        Grid {
            data: vec![value; width * height],
            cols: width,
        }
    }

    pub fn replace(&mut self, point: Vec2, value: T) -> Option<T> {
        self.get_mut(point).map(|dest| mem::replace(dest, value))
    }

    pub fn map<U: Clone>(&self, value: U) -> Grid<U> {
        Grid::new(self.width(), self.height(), value)
    }

    pub fn swap(&mut self, a: Vec2, b: Vec2)
    where
        T: Clone,
    {
        let i = self.to_index(a).unwrap();
        let j = self.to_index(b).unwrap();
        self.data.swap(i, j);
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.cols
    }

    pub fn width(&self) -> usize {
        self.cols
    }

    pub fn points(&self) -> impl Iterator<Item = Vec2> {
        let rows = self.height() as i32;
        let cols = self.width() as i32;
        (0..rows).flat_map(move |y| (0..cols).map(move |x| Vec2 { x, y }))
    }

    pub fn get(&self, point: Vec2) -> Option<&T> {
        self.to_index(point).and_then(|i| self.data.get(i))
    }

    pub fn get_mut(&mut self, point: Vec2) -> Option<&mut T> {
        self.to_index(point).and_then(move |i| self.data.get_mut(i))
    }

    pub fn has(&self, point: Vec2) -> bool {
        self.to_index(point).map_or(false, |i| i < self.data.len())
    }

    pub fn find(&self, filter: impl Fn(&T) -> bool) -> Option<Vec2> {
        let pos = self.data.iter().position(filter)?;
        Some(Vec2 {
            x: (pos % self.cols) as i32,
            y: (pos / self.cols) as i32,
        })
    }

    pub fn neighbors(&self, point: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        Dir::all()
            .map(move |dir| point.neighbor(dir))
            .filter(move |&p| self.has(p))
    }

    fn to_index(&self, Vec2 { x, y }: Vec2) -> Option<usize> {
        if x >= 0 && y >= 0 {
            Some(y as usize * self.cols + x as usize)
        } else {
            None
        }
    }
}

impl<T> Index<Vec2> for Grid<T> {
    type Output = T;

    fn index(&self, point: Vec2) -> &Self::Output {
        let index = self.to_index(point).unwrap();
        &self.data[index]
    }
}

impl<T> IndexMut<Vec2> for Grid<T> {
    fn index_mut(&mut self, point: Vec2) -> &mut Self::Output {
        let index = self.to_index(point).unwrap();
        &mut self.data[index]
    }
}