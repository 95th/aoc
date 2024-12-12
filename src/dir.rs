use crate::dist::Dist;

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

    pub const fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub const fn into_distance(self) -> Dist {
        match self {
            Self::Up => Dist(-1, 0),
            Self::Down => Dist(1, 0),
            Self::Left => Dist(0, -1),
            Self::Right => Dist(0, 1),
        }
    }
}
