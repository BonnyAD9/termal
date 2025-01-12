use std::ops::Sub;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Vec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn pos_of_idx(&self, idx: usize) -> Self {
        (idx % self.x, idx / self.x).into()
    }

    pub fn map(mut self, mut f: impl FnMut(usize) -> usize) -> Self {
        self.x = f(self.x);
        self.y = f(self.y);
        self
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl PartialEq<(usize, usize)> for Vec2 {
    fn eq(&self, (x, y): &(usize, usize)) -> bool {
        self.x == *x && self.y == *y
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}
