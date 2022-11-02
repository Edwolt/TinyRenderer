use std::ops::Mul;

use super::Vector3;

#[derive(Debug, Clone)]
pub struct Matrix {
    /// rows
    n: usize,
    /// columns
    m: usize,
    data: Vec<f64>,
}

/// Create a matrix
///
/// Create a matrix 3x4 filled with the value 2:
/// ```
/// mat![2.0; 3, 4]
/// ```
///
/// <pre>
/// Create a matrix 3x4 with the values
/// |0, 1, 2|
/// |6, 5, 4|
/// |7, 8, 9|
/// </pre>
///
/// ```
/// mat![3; 4 =>
///     0.0, 1.0, 2.0;
///     6.0, 5.0, 4.0;
///     7.0, 8.0, 9.0;
/// ]
/// ```
#[macro_export]
macro_rules! mat {
    ($elem:expr; $n:expr; $m:expr) => {
        Matrix::new(vec![$elem; $n * $m], $n, $m)
    };
    ($n:expr, $m:expr => $($($x:expr), *); *) => (
        Matrix::new(vec![$($($x), *), *], $n, $m)
    );
}

impl Matrix {
    pub fn new(data: Vec<f64>, n: usize, m: usize) -> Self {
        assert_eq!(
            data.len(),
            n * m,
            "A Matrix {}x{} must have a data vec  of the size {}",
            n,
            m,
            n * m
        );
        Self { n, m, data }
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.m + j]
    }
    pub fn set(&mut self, i: usize, j: usize, value: f64) {
        self.data[i * self.m + j] = value;
    }

    pub fn transpose(mut self) ->Self{
        for i in 1..self.n {
            for j in 0..i {
                // swap(self[i][j], self[j][i])
                let aux = self.get(i, j);
                self.set(i, j, self.get(j, i));
                self.set(j, i, aux);
            }
        }
        self
    }

    /// Convert a matrix 4x1 that represents to a vertex3
    ///
    /// mat![4, 1 => x; y; z; w] -> (x/w, y/w, z/w)
    pub fn to_vector3(&self) -> Vector3 {
        assert!(self.n == 4 && self.m == 1, "Matrix must be 4x1");
        let w = self.get(3, 0);
        if w < f64::EPSILON {
            // w = 0 => A Vector
            Vector3 {
                x: self.get(0, 0),
                y: self.get(1, 0),
                z: self.get(2, 0),
            }
        } else {
            // w != 0 => A Point
            Vector3 {
                x: self.get(0, 0) / w,
                y: self.get(1, 0) / w,
                z: self.get(2, 0) / w,
            }
        }
    }
}

// Matrix * Matrix
impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Self::Output {
        assert_eq!(
            self.m, other.n,
            "Can't multiply a Matrix {}x{} with one {}x{}",
            self.n, self.m, other.n, other.m
        );
        let mut data: Vec<f64> = Vec::new();
        for i in 0..self.n {
            for j in 0..other.m {
                let mut val = 0.0;
                for k in 0..self.m {
                    val += self.get(i, k) * other.get(k, j)
                }
                data.push(val);
            }
        }

        Matrix::new(data, self.n, other.m)
    }
}

// Matrix * &Matrix
impl Mul<&Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Self::Output {
        assert_eq!(
            self.m, other.n,
            "Can't multiply a Matrix {}x{} with one {}x{}",
            self.n, self.m, other.n, other.m
        );
        let mut data: Vec<f64> = Vec::new();
        for i in 0..self.n {
            for j in 0..other.m {
                let mut val = 0.0;
                for k in 0..self.m {
                    val += self.get(i, k) * other.get(k, j)
                }
                data.push(val);
            }
        }

        Matrix::new(data, self.n, other.m)
    }
}

// &Matrix * &Matrix
impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Self::Output {
        assert_eq!(
            self.m, other.n,
            "Can't multiply a Matrix {}x{} with one {}x{}",
            self.n, self.m, other.n, other.m
        );
        let mut data: Vec<f64> = Vec::new();
        for i in 0..self.n {
            for j in 0..other.m {
                let mut val = 0.0;
                for k in 0..self.m {
                    val += self.get(i, k) * other.get(k, j)
                }
                data.push(val);
            }
        }

        Matrix::new(data, self.n, other.m)
    }
}

// &Matrix * Matrix
impl Mul<Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Self::Output {
        assert_eq!(
            self.m, other.n,
            "Can't multiply a Matrix {}x{} with one {}x{}",
            self.n, self.m, other.n, other.m
        );
        let mut data: Vec<f64> = Vec::new();
        for i in 0..self.n {
            for j in 0..other.m {
                let mut val = 0.0;
                for k in 0..self.m {
                    val += self.get(i, k) * other.get(k, j)
                }
                data.push(val);
            }
        }

        Matrix::new(data, self.n, other.m)
    }
}
