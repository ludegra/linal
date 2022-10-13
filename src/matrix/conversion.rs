use crate::vector::Vector;

use super::Matrix;

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

impl<const M: usize, T> From<Matrix<M, 1, T>> for Vector<M, T>
where
    T: Default + Copy,
{
    fn from(mat: Matrix<M, 1, T>) -> Self {
        let mut out = [T::default(); M];
        for m in 0..M {
            out[m] = mat[m][0];
        }
        Self::from(out)
    }
}
