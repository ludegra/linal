use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign, Div};

use crate::vector::Vector;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Matrix<const M: usize, const N: usize, T> {
    matrix: [[T; N]; M],
}

impl<const M: usize, const N: usize, T> Matrix<M, N, T> {
    pub fn iter(&self) -> std::slice::Iter<[T; N]> {
        self.matrix.iter()
    }
}
impl<const M: usize, const N: usize, T> Matrix<M, N, T>
where
    T: Copy + Default,
{
    pub fn transpose(self) -> Matrix<N, M, T> {
        let mut out = [[T::default(); M]; N];

        for m in 0..M {
            for n in 0..N {
                out[m][n] = self[n][m];
            }
        }

        Matrix::from(out)
    }
}
impl<const M: usize, const N: usize, T> Matrix<M, N, T>
where
    T: Copy + Default + Div + From<u8>
{
    pub fn rref(mut self) {
        let matrix = self.matrix;

        // TODO: Implement RREF function
        todo!()
    }
}

impl<const M: usize, T> Matrix<M, M, T>
where
    T: Copy + Default + From<u8>,
{
    pub fn ident() -> Self {
        let mut out = [[T::default(); M]; M];
        for m in 0..M {
            out[m][m] = T::from(1);
        }
        Self::from(out)
    }
}

// Trait implementations

// Index implementations
impl<const M: usize, const N: usize, T> Index<usize> for Matrix<M, N, T> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}
impl<const M: usize, const N: usize, T> IndexMut<usize> for Matrix<M, N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

// IntoIterator implementation
impl<const M: usize, const N: usize, T> IntoIterator for Matrix<M, N, T> {
    type Item = [T; N];
    type IntoIter = std::array::IntoIter<Self::Item, M>;

    fn into_iter(self) -> Self::IntoIter {
        self.matrix.into_iter()
    }
}

// Arithmetic implementations

// Addition
impl<const M: usize, const N: usize, T> Add<Self> for Matrix<M, N, T>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = [[T::default(); N]; M];

        for m in 0..M {
            for n in 0..N {
                out[m][n] = self[m][n] + rhs[m][n];
            }
        }

        Self::from(out)
    }
}
impl<const M: usize, const N: usize, T> AddAssign<Self> for Matrix<M, N, T>
where
    T: Add<Output = T> + AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        for m in 0..M {
            for n in 0..N {
                self[m][n] += rhs[m][n];
            }
        }
    }
}

// Subtraction
impl<const M: usize, const N: usize, T> Sub<Self> for Matrix<M, N, T>
where
    T: Sub<Output = T> + Default + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = [[T::default(); N]; M];

        for m in 0..M {
            for n in 0..N {
                out[m][n] = self[m][n] - rhs[m][n];
            }
        }

        Self::from(out)
    }
}
impl<const M: usize, const N: usize, T> SubAssign<Self> for Matrix<M, N, T>
where
    T: Add + Sub<Output = T> + SubAssign + Mul + Default + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        for m in 0..M {
            for n in 0..N {
                self[m][n] -= rhs[m][n];
            }
        }
    }
}

// Multiplication with scalar
impl<const M: usize, const N: usize, T> Mul<T> for Matrix<M, N, T>
where
    T: Mul<Output = T> + Default + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut out = [[T::default(); N]; M];

        for m in 0..M {
            for n in 0..N {
                out[m][n] = self[m][n] * rhs;
            }
        }

        Self::from(out)
    }
}
impl<const M: usize, const N: usize, T> MulAssign<Self> for Matrix<M, N, T>
where
    T: Mul<Output = T> + MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        for m in 0..M {
            for n in 0..N {
                self[m][n] *= rhs[m][n];
            }
        }
    }
}

// Multiplication with matrix
impl<const M: usize, const N: usize, const P: usize, T> Mul<Matrix<N, P, T>> for Matrix<M, N, T>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    type Output = Matrix<M, P, T>;

    fn mul(self, rhs: Matrix<N, P, T>) -> Self::Output {
        let mut out = [[T::default(); P]; M];

        for m in 0..M {
            for p in 0..P {
                out[m][p] = (0..N).fold(T::default(), |acc, n| self[m][n] * rhs[n][p] + acc)
            }
        }

        Self::Output::from(out)
    }
}

// From implementations
impl<const M: usize, const N: usize, T> From<[[T; N]; M]> for Matrix<M, N, T> {
    fn from(matrix: [[T; N]; M]) -> Self {
        Self { matrix }
    }
}

impl<const M: usize, T> From<Vector<M, T>> for Matrix<M, 1, T>
where
    T: Default + Copy,
{
    fn from(vec: Vector<M, T>) -> Self {
        let mut out = [[T::default()]; M];
        for m in 0..M {
            out[m] = [vec[m]];
        }
        Self::from(out)
    }
}
