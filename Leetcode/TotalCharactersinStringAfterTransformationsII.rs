// https://leetcode.com/problems/total-characters-in-string-after-transformations-ii

pub mod matrix {
    #[derive(Debug, Clone)]
    pub struct Matrix<const MOD: usize> {
        data: Vec<Vec<usize>>,
        rows: usize,
        cols: usize,
    }

    impl<const MOD: usize> Matrix<MOD> {
        pub fn new(data: Vec<Vec<usize>>) -> Self {
            assert!(!data.is_empty());
            assert!(!data[0].is_empty());

            let rows = data.len();
            let cols = data[0].len();

            Matrix { data, rows, cols }
        }

        pub fn identity(n: usize) -> Self {
            let mut data = vec![vec![0; n]; n];
            for i in 0..n {
                data[i][i] = 1;
            }

            Matrix::new(data)
        }

        pub fn multiply(&self, other: &Matrix<MOD>) -> Matrix<MOD> {
            assert_eq!(
                self.cols, other.rows,
                "Matrix dimensions do not match for multiplication"
            );

            let mut result = vec![vec![0; other.cols]; self.rows];
            for i in 0..self.rows {
                for j in 0..other.cols {
                    for k in 0..self.cols {
                        result[i][j] += self.data[i][k] * other.data[k][j];
                        result[i][j] %= MOD;
                    }
                }
            }

            Matrix::new(result)
        }

        pub fn pow(&self, mut exp: u64) -> Matrix<MOD> {
            assert_eq!(
                self.rows, self.cols,
                "Only square matrices can be exponentiated"
            );

            let mut base = self.clone();
            let mut result = Matrix::identity(self.rows);

            while exp > 0 {
                if exp % 2 == 1 {
                    result = result.multiply(&base);
                }
                base = base.multiply(&base);
                exp /= 2;
            }

            result
        }

        pub fn elemements_sum(&self) -> usize {
            let mut result = 0;
            for row in &self.data {
                for &elem in row {
                    result += elem;
                    result %= MOD;
                }
            }

            result
        }
    }
}

struct Solution {}

impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        const MOD: usize = 1_000_000_000 + 7;

        let mut T = vec![vec![0; 26]; 26];
        for c in 0..26 {
            for dc in 1..=nums[c] as usize {
                T[(c + dc) % 26][c] = 1;
            }
        }
        let T = matrix::Matrix::<MOD>::new(T);
        let T = T.pow(t as u64);

        let mut B = vec![vec![0]; 26];
        for c in s.into_bytes().into_iter().map(|b| (b - b'a') as usize) {
            B[c][0] += 1;
        }
        let B = matrix::Matrix::<MOD>::new(B);

        let R = T.multiply(&B);
        R.elemements_sum() as i32
    }
}
