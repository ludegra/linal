#![feature(generic_const_exprs)]

pub mod matrix;
pub mod vector;

#[cfg(test)]
mod tests {
    mod matrix {
        use crate::matrix::{Matrix, Total};

        #[test]
        fn matrix_addition() {
            let m1 = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
            let m2 = Matrix::from([[9, 8, 7], [6, 5, 4], [3, 2, 1]]);
            let m3 = Matrix::from([[10, 10, 10], [10, 10, 10], [10, 10, 10]]);

            assert_eq!(m1 + m2, m3);
        }

        #[test]
        fn matrix_subtraction() {
            let m1 = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
            let m2 = Matrix::from([[0, 1, 2], [3, 4, 5], [6, 7, 8]]);
            let m3 = Matrix::from([[1, 1, 1], [1, 1, 1], [1, 1, 1]]);

            assert_eq!(m1 - m2, m3);
        }

        #[test]
        fn matrix_scalar_multiplication() {
            let m1 = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
            let m2 = Matrix::from([[2, 4, 6], [8, 10, 12], [14, 16, 18]]);

            assert_eq!(m1 * 2, m2);
        }

        #[test]
        fn matrix_matrix_multiplication() {
            let m1 = Matrix::from([[1, 2, 3], [4, 5, 6]]);
            let m2 = Matrix::from([[1, 2], [4, 5], [7, 8]]);
            let m3 = Matrix::from([[30, 36], [66, 81]]);

            assert_eq!(m1 * m2, m3)
        }

        #[test]
        fn matrix_determinant() {
            let m1 = Matrix::<3, 3, f32>::from([[8.0, 3.0, 2.0], [1.0, 7.0, 9.0], [5.0, 3.0, 3.0]]);

            assert_eq!(m1.det(), 14.0);
        }

        #[test]
        fn matrix_inverse() {
            let m1 = Matrix::<3, 3, f32>::from([[1.0, 2.0, -2.0], [1.0, 4.0, 1.0], [0.0, 1.0, 2.0]]);
            let m2 = Matrix::<3, 3, f32>::from([[7.0, -6.0, 10.0], [-2.0, 2.0, -3.0], [1.0, -1.0, 2.0]]);

            assert!(m1.inverse().is_some());
            assert_eq!(m1.inverse().unwrap(), m2)
        }

        #[test]
        fn matrix_total() {
            let m1 = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
            let m2 = Matrix::from([[3, 2, 1], [6, 5, 4], [9, 8, 7]]);
            let m3 = Matrix::from([[1, 2, 3, 3, 2, 1], [4, 5, 6, 6, 5, 4], [7, 8, 9, 9, 8, 7]]);

            assert_eq!(Matrix::total(m1, m2), m3)
        }
    }
}
