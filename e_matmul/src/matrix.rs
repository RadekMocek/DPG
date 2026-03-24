use rand::distr::Distribution;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(PartialEq)]
pub(super) struct SquareMx {
    pub items: Vec<u32>,
    pub dim: usize,
}

impl SquareMx {
    pub(super) fn zeroes_square(dim: usize) -> Self {
        Self {
            items: vec![0; dim * dim],
            dim,
        }
    }

    pub(super) fn random_square(dim: usize, from: u32, to: u32) -> Self {
        let mut rng = rand::rng();
        let range = rand::distr::Uniform::try_from(from..to).unwrap();
        Self {
            items: (0..(dim * dim)).map(|_| range.sample(&mut rng)).collect(),
            dim,
        }
    }

    pub(super) fn get_transposed(&self) -> SquareMx {
        let mut result = Self::zeroes_square(self.dim);
        for row_n in 0..self.dim {
            for col_n in 0..self.dim {
                result[(col_n, row_n)] = self[(row_n, col_n)];
            }
        }
        result
    }

    pub(super) fn get_row_slice(&self, row_n: usize) -> &[u32] {
        let row_start = row_n * self.dim;
        &self.items[row_start..(row_start + self.dim)]
    }
}

impl Index<(usize, usize)> for SquareMx {
    type Output = u32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.items[index.0 * self.dim + index.1]
    }
}

impl IndexMut<(usize, usize)> for SquareMx {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.items[index.0 * self.dim + index.1]
    }
}

impl Display for SquareMx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for row_n in 0..self.dim {
            if row_n != 0 {
                write!(f, ";")?;
            }
            for col_n in 0..self.dim {
                if col_n != 0 {
                    write!(f, ",")?;
                }
                write!(f, "{}", self[(row_n, col_n)])?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}
