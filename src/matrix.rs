use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

mod arithmetic;
mod conversion;
mod fmt;
mod iter;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Matrix<const M: usize, const N: usize, T> {
    matrix: [[T; N]; M],
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
    T: Copy
        + Default
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + Div<Output = T>
        + DivAssign
        + From<i8>
        + PartialEq,
{
    pub fn rref(mut self) -> Matrix<M, N, T> {
        let mut row = 0;
        for collumn in 0..N {
            if row >= M {
                break;
            }

            // Check for first row with non-zero value
            if self.matrix[row][collumn] == T::default() {
                let mut swapped = false;
                for i in (row + 1)..M {
                    if self.matrix[i][collumn] != T::default() {
                        self.matrix.swap(collumn, i);
                        swapped = true;
                        break;
                    }
                }

                // Skip collumn if no non-zero value found
                if !swapped {
                    continue;
                }
            }

            // Divide row by first non-zero value
            let pivot = self.matrix[row][collumn];

            for i in collumn..N {
                self.matrix[row][i] /= pivot;
            }

            // Subtract row from all other rows
            for i in 0..M {
                if i == row {
                    continue;
                }

                let factor = self.matrix[i][collumn];

                for j in collumn..N {
                    self.matrix[i][j] -= factor * self.matrix[row][j];
                }
            }

            row += 1;
        }

        self
    }
}

impl<const M: usize, T> Matrix<M, M, T>
where
    T: Copy + Default + From<i8>,
{
    pub fn ident() -> Self {
        let mut out = [[T::default(); M]; M];
        for m in 0..M {
            out[m][m] = T::from(1);
        }
        Self::from(out)
    }
}

impl<const M: usize, T> Matrix<M, M, T>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + PartialEq
        + Copy
        + Default
        + From<i8>,
    [(); M + M]:,
{
    pub fn inverse(&self) -> Option<Self> {
        if self.det() == T::default() {
            return None;
        }

        let total = Matrix::total(*self, Self::ident());
        let reduced = total.rref();

        let mut out = [[T::default(); M]; M];
        for m in 0..M {
            for n in 0..M {
                out[m][n] = reduced[m][n + M];
            }
        }

        Some(Self::from(out))
    }
}

impl<const M: usize, T> Matrix<M, M, T>
where
    T: AddAssign + Sub<Output = T> + Mul<Output = T> + Default + Copy + From<i8>,
{
    pub fn det(&self) -> T {
        Self::det_helper(
            self.matrix
                .into_iter()
                .map(|row| row.into_iter().collect())
                .collect(),
        )
    }

    fn det_helper(vec: Vec<Vec<T>>) -> T {
        if vec.len() == 2 {
            return vec[0][0] * vec[1][1] - vec[0][1] * vec[1][0];
        }

        let mut out = T::default();
        for i in 0..vec.len() {
            let mut sub = vec.clone();
            sub.remove(0);
            for j in 0..sub.len() {
                sub[j].remove(i);
            }

            out += vec[0][i] * Self::det_helper(sub) * if i % 2 == 0 { T::from(1) } else { T::from(-1) };
        }

        out
    }
}

impl<const M: usize, T> Matrix<M, M, T>
where
    T: Add<Output = T> + AddAssign + Sub<Output = T> + Mul<Output = T> + From<i8> + Default + Copy,
    [(); M - 1]:,
{
    pub fn sub_matrix(&self, row: usize, collumn: usize) -> Matrix<{ M - 1 }, { M - 1 }, T> {
        let mut out = [[T::default(); { M - 1 }]; { M - 1 }];

        for m in 0..M {
            if m == row {
                continue;
            }

            for n in 0..M {
                if n == collumn {
                    continue;
                }

                out[m + (m > row) as usize][n + (n > collumn) as usize] = self[m][n];
            }
        }

        Matrix::from(out)
    }
}

pub trait Total<const M: usize, const N: usize, const R: usize, T>
where
    T: Copy,
{
    fn total(lhs: Matrix<M, N, T>, rhs: Matrix<M, R, T>) -> Matrix<M, { N + R }, T>;
}

impl<const M: usize, const N: usize, const R: usize, T> Total<M, N, R, T>
    for Matrix<M, { N + R }, T>
where
    T: Copy + Default,
{
    fn total(lhs: Matrix<M, N, T>, rhs: Matrix<M, R, T>) -> Self {
        let mut out = [[T::default(); N + R]; M];

        for m in 0..M {
            for n in 0..N {
                out[m][n] = lhs[m][n]
            }
            for r in 0..R {
                out[m][r + N] = rhs[m][r]
            }
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
