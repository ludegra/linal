use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
    slice::Iter,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector<const M: usize, T> {
    vec: [T; M],
}
impl<const M: usize, T> Vector<M, T> {
    pub fn vec(self) -> [T; M] {
        self.vec
    }
    pub fn iter(&self) -> Iter<T> {
        self.vec.iter()
    }
}
impl<const M: usize, T> Vector<M, T>
where
    T: Add + Copy + Into<f64>,
{
    pub fn magnitude(&self) -> f64 {
        self.vec
            .into_iter()
            .fold(0.0, |acc, v| acc + f64::powi(v.into(), 2))
            .sqrt()
    }
}

impl<T> Vector<3, T>
where
    T: Sub<Output = T> + Mul<Output = T> + Default + Copy,
{
    pub fn crossp(&mut self, rhs: &Self) -> Self {
        let mut out = [T::default(); 3];

        out[0] = self[1] * rhs[2] - self[2] * rhs[1];
        out[1] = self[2] * rhs[0] - self[0] * rhs[2];
        out[2] = self[0] * rhs[1] - self[1] * rhs[0];

        Self::from(out)
    }
}

// Trait implementations

//Index implementations
impl<const M: usize, T> Index<usize> for Vector<M, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}
impl<const M: usize, T> IndexMut<usize> for Vector<M, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}

// Arithmetic implementations

// Addition
impl<const M: usize, T> Add<Self> for Vector<M, T>
where
    T: Add<Output = T> + Copy + Default,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = [T::default(); M];

        for m in 0..M {
            out[m] = self[m] + rhs[m];
        }

        Self::from(out)
    }
}
impl<const M: usize, T> AddAssign<Self> for Vector<M, T>
where
    T: Add<Output = T> + AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        for m in 0..M {
            self[m] += rhs[m];
        }
    }
}

// Subtraction
impl<const M: usize, T> Sub<Self> for Vector<M, T>
where
    T: Sub<Output = T> + Copy + Default,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = [T::default(); M];

        for m in 0..M {
            out[m] = self[m] - rhs[m];
        }

        Self::from(out)
    }
}
impl<const M: usize, T> SubAssign<Self> for Vector<M, T>
where
    T: Sub<Output = T> + SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        for m in 0..M {
            self[m] -= rhs[m]
        }
    }
}

// Multiplication by scalar
impl<const M: usize, T> Mul<T> for Vector<M, T>
where
    T: Mul<Output = T> + Copy + Default,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut out = [T::default(); M];

        for m in 0..M {
            out[m] = self[m] * rhs;
        }

        Self::from(out)
    }
}
impl<const M: usize, T> MulAssign<T> for Vector<M, T>
where
    T: Mul<Output = T> + MulAssign + Copy + Default,
{
    fn mul_assign(&mut self, rhs: T) {
        for m in 0..M {
            self[m] *= rhs
        }
    }
}

// Multiplication by vector
impl<const M: usize, T> Mul<Self> for Vector<M, T>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = T::default();

        for m in 0..M {
            out += self[m] * rhs[m]
        }

        out
    }
}

// From implementations
impl<const M: usize, T> From<[T; M]> for Vector<M, T> {
    fn from(arr: [T; M]) -> Self {
        Self { vec: arr }
    }
}
