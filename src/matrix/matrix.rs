extern crate num_traits;
use num_traits::Float;
//use std::fmt;
//use std::ops;

#[derive(Debug, PartialEq)]
pub struct Matrix<T: Float, const ROWS: usize, const COLS: usize> {
    data: [[T; ROWS]; COLS],
}

impl<T: Float, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn add(&self, other: &Matrix<T, ROWS, COLS>) -> Matrix<T, ROWS, COLS> {
        let mut result = [[T::zero(); ROWS]; COLS];
        for y in 0..ROWS {
            for x in 0..COLS {
                result[y][x] = self.data[y][x] + other.data[y][x];
            }
        }
        return Matrix::<T, ROWS, COLS> { data: result };
    }

    pub fn sub(&self, other: &Matrix<T, ROWS, COLS>) -> Matrix<T, ROWS, COLS> {
        let mut result = [[T::zero(); ROWS]; COLS];
        for y in 0..ROWS {
            for x in 0..COLS {
                result[y][x] = self.data[y][x] - other.data[y][x];
            }
        }
        return Matrix::<T, ROWS, COLS> { data: result };
    }

    pub fn scale(&self, scalar: T) -> Matrix<T, ROWS, COLS> {
        let mut result = [[T::zero(); ROWS]; COLS];
        for y in 0..ROWS {
            for x in 0..COLS {
                result[y][x] = self.data[y][x] * scalar;
            }
        }
        return Matrix::<T, ROWS, COLS> { data: result };
    }

    pub fn transpose(&self) -> Matrix<T, COLS, ROWS> {
        let mut result = [[T::zero(); COLS]; ROWS];
        for y in 0..ROWS {
            for x in 0..COLS {
                result[x][y] = self.data[y][x];
            }
        }
        return Matrix::<T, COLS, ROWS> { data: result };
    }
}

// A M by N matrix times a N by K matrix results in a M by K product
// impl<T: Float, const M: usize, const N: usize, const K: usize> Matrix<T, M, N> {
impl<T: Float, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn multiply<const K: usize>(&self, other: Matrix<T, N, K>) -> Matrix<T, M, K> {
        let mut result = [[T::zero(); M]; K];

        for i in 0..M {
            for j in 0..N {
                for k in 0..K {
                    result[i][j] = result[i][j] + self.data[i][k] * other.data[k][j];
                }
            }
        }
        return Matrix::<T, M, K> { data: result };
    }
}

// Square matrix specific stuff
impl<T: Float, const DIM: usize> Matrix<T, DIM, DIM> {
    pub fn identity() -> Matrix<T, DIM, DIM> {
        let mut result = [[T::zero(); DIM]; DIM];
        for i in 0..DIM {
            result[i][i] = T::one();
        }
        return Matrix::<T, DIM, DIM> { data: result };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn identity() {
        assert_eq!(
            Matrix::<f32, 1, 1>::identity(),
            Matrix::<f32, 1, 1> {
                data: [[1.0; 1]; 1]
            }
        );
        assert_eq!(
            Matrix::<f32, 2, 2>::identity(),
            Matrix::<f32, 2, 2> {
                data: [[1.0, 0.0], [0.0, 1.0]]
            }
        );
        assert_eq!(
            Matrix::<f32, 3, 3>::identity(),
            Matrix::<f32, 3, 3> {
                data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
            }
        );
    }
}
