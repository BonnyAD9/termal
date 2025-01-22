use std::ops::{Index, IndexMut};

use crate::Rgb;

use super::Image;

/// Fixed size matrix. Wrapper around [`Vec`]
pub struct Mat<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Mat<T> {
    /// Create new matrix with the given size.
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        let mut data = vec![];
        data.resize_with(width * height, Default::default);
        Self {
            width,
            height,
            data,
        }
    }

    /// Create new fixed size matrix from the given data with the given size.
    ///
    /// # Panics
    /// - If the vector size doesn't match width and height.
    pub fn from_vec(width: usize, height: usize, data: Vec<T>) -> Self {
        if data.len() != width * height {
            panic!(
                "Invalid Mat data length of {} for [{width}, {height}]({})",
                data.len(),
                width * height
            );
        }
        Self {
            width,
            height,
            data,
        }
    }

    /// Get the width of the matrix.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height of the matrix.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get one line of the matrix.
    pub fn get_line(&self, y: usize) -> &[T] {
        let start = y * self.width;
        &self.data[start..start + self.width]
    }
}

impl Image for Mat<Rgb> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_pixel(&self, x: usize, y: usize) -> Rgb {
        self[(x, y)]
    }
}

impl<T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x > self.width || y > self.height {
            panic!(
                "Mat index [{x}, {y}] out of range of [{}, {}]",
                self.width, self.height
            );
        }
        &self.data[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x > self.width || y > self.height {
            panic!(
                "Mat index [{x}, {y}] out of range of [{}, {}]",
                self.width, self.height
            );
        }
        &mut self.data[y * self.width + x]
    }
}
