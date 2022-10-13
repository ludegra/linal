use super::Matrix;

impl<const M: usize, const N: usize, T> Matrix<M, N, T> {
    pub fn iter(&self) -> std::slice::Iter<[T; N]> {
        self.matrix.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<[T; N]> {
        self.matrix.iter_mut()
    }
}

impl<const M: usize, const N: usize, T> IntoIterator for Matrix<M, N, T> {
    type Item = [T; N];
    type IntoIter = std::array::IntoIter<Self::Item, M>;

    fn into_iter(self) -> Self::IntoIter {
        self.matrix.into_iter()
    }
}