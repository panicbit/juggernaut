extern crate rand;

use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(Vec<Vec<f64>>);

pub trait MatrixTrait {
    fn zero(m: usize, n: usize) -> Self;
    fn random(m: usize, n: usize) -> Self;
    fn generate(m: usize, n: usize, f: &Fn() -> f64) -> Self;
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn get(&self, m: usize, n: usize) -> f64;
    fn dot(&self, b: Matrix) -> Matrix;
}

impl MatrixTrait for Matrix {
    /// Returns a vector with `m` rows and `n` columns
    fn generate(m: usize, n: usize, f: &Fn() -> f64) -> Matrix {
        let mut mtx: Vec<Vec<f64>> = Vec::with_capacity(m);

        for _ in 0..m {
            let mut row: Vec<f64> = Vec::with_capacity(n);

            for _ in 0..n {
                row.push(f());
            }

            mtx.push(row);
        }

        Matrix(mtx)
    }

    /// Returns a vector with `m` rows and `n` columns with elements of 0
    fn zero(m: usize, n: usize) -> Matrix {
        Matrix::generate(m, n, &|| 0f64)
    }

    /// Returns a vector with `m` rows and `n` columns with random elements
    fn random(m: usize, n: usize) -> Matrix {
        Matrix::generate(m, n, &|| rand::thread_rng().gen_range(-1f64, 1f64))
    }

    /// Number of the Matrix rows
    fn rows(&self) -> usize {
        self.0.len()
    }

    /// Number of the Matrix columns
    fn cols(&self) -> usize {
        self.0[0].len()
    }

    /// Returns the element in the position M,N
    fn get(&self, m: usize, n: usize) -> f64 {
        assert!(self.rows() >= m && self.cols() >= n);

        self.0[m][n]
    }

    /// Multiplication with Matrix
    fn dot(&self, b: Matrix) -> Matrix {
        assert_eq!(self.rows(), b.cols());

        let mut result: Matrix = Matrix::zero(self.rows(), b.cols());

        for (m, row) in self.0.iter().enumerate() {
            for n in 0usize..b.cols() {
                let mut cell_result: f64 = 0f64;

                for (k, row_cell) in row.iter().enumerate() {
                    // row of the first Matrix X col of the second Matrix
                    cell_result += row_cell * b.get(k, n);
                }

                result.0[m][n] = cell_result;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_matrix_test() {
        let test = Matrix(vec![vec![0f64, 0f64], vec![0f64, 0f64]]);
        assert_eq!(Matrix::zero(2, 2), test);
    }

    #[test]
    fn random_matrix_test() {
        let test = Matrix::random(2, 2);

        assert_ne!(test.0[0][0], test.0[0][1]);
        assert_ne!(test.0[1][0], test.0[1][1]);
    }

    #[test]
    fn random_matrix_get() {
        let test = Matrix::random(2, 2);

        assert_approx_eq!(test.get(0, 1), test.0[0][1]);
        assert_approx_eq!(test.get(1, 0), test.0[1][0]);
        assert_approx_eq!(test.get(1, 1), test.0[1][1]);
        assert_approx_eq!(test.get(0, 0), test.0[0][0]);
    }

    #[test]
    fn random_mul_test1() {
        let a = Matrix(vec![vec![1f64, 2f64], vec![3f64, 4f64]]);
        let b = Matrix(vec![vec![2f64, 0f64], vec![1f64, 2f64]]);
        let result = Matrix(vec![vec![4f64, 4f64], vec![10f64, 8f64]]);

        assert_eq!(a.dot(b), result);
    }

    #[test]
    fn random_mul_test2() {
        let a = Matrix(vec![vec![1f64, 2f64], vec![3f64, 4f64]]);
        let b = Matrix(vec![vec![2f64, 0f64], vec![1f64, 2f64]]);
        let result = Matrix(vec![vec![2f64, 4f64], vec![7f64, 10f64]]);

        assert_eq!(b.dot(a), result);
    }
}