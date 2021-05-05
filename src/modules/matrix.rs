#[derive(Debug)]
pub struct Matrix<T> {
    n: usize,
    m: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<T>, n: usize, m: usize) -> Matrix<T> {
        // TODO verify if size is ok
        Matrix { n, m, data }
    }
}

#[macro_export]
macro_rules! mat {
    ($elem:expr; $n:expr; $m:expr) => {
        Matrix::new(vec![$elem; $n * $m], $n, $m)
    };
    ($n:expr, $m:expr => $($($x:expr), *); *) => (
        Matrix::new(vec![$($($x), *), *], $n, $m)
    );
}
