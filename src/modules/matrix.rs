use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix {
    /// rows
    n: usize,
    /// columns
    m: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(data: Vec<f64>, n: usize, m: usize) -> Matrix {
        assert_eq!(data.len(), n * m, "Matrix: worng data vec length");
        Matrix { n, m, data }
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        // assert!(i < self.n, "Matrix: Invalid i");
        // assert!(j < self.n, "Matrix: Invalid j");
        self.data[i * self.m + j]
    }
    pub fn set(&mut self, i: usize, j: usize, value: f64) {
        // assert!(i < self.n, "Matrix: i>=n (i={}, n={})", i, self.n);
        // assert!(j < self.n, "Matrix: j>=m (j={}, m={})", j, self.m);
        self.data[i * self.m + j] = value;
    }
}

impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Matrix {
        assert_eq!(self.m, other.n, "Matrix: Can't multiply");
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

#[macro_use]
macro_rules! mat {
    ($elem:expr; $n:expr; $m:expr) => {
        Matrix::new(vec![$elem; $n * $m], $n, $m)
    };
    ($n:expr, $m:expr => $($($x:expr), *); *) => (
        Matrix::new(vec![$($($x), *), *], $n, $m)
    );
}
