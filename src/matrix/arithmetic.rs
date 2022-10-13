use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

use super::Matrix;

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