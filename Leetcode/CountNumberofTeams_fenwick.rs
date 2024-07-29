// https://leetcode.com/problems/count-number-of-teams

mod cm_fenwick {
    pub struct Fenwick<T> {
        ary: Vec<T>,
    }

    impl<T: Clone + Default + std::ops::AddAssign<T>> Fenwick<T> {
        /// Creates a new Fenwick Tree with size n
        ///
        /// # Complexity
        /// - Time: O(n)
        /// - Space: O(n)
        pub fn new(n: usize) -> Self {
            Fenwick {
                ary: vec![T::default(); n],
            }
        }

        /// Creates a Fenwick Tree on a given array
        ///
        /// # Complexity
        /// - Time: O(n)
        /// - Space: O(n)
        pub fn build_on_array(a: &[T]) -> Self {
            let mut ary = a.to_vec();
            for i in 0..a.len() {
                let j = i | (i + 1);
                if j < a.len() {
                    let tmp = ary[i].clone();
                    ary[j] += tmp;
                }
            }
            Fenwick { ary }
        }

        fn accum(&self, mut idx: usize) -> T {
            let mut sum = T::default();
            while idx > 0 {
                sum += self.ary[idx - 1].clone();
                idx &= idx - 1;
            }
            sum
        }

        /// Increments the value at `idx` by `val`
        ///
        /// # Complexity
        /// - Time: O(log n)
        /// - Space: O(1)
        pub fn add(&mut self, mut idx: usize, val: T) {
            while idx < self.ary.len() {
                self.ary[idx] += val.clone();
                idx |= idx + 1;
            }
        }

        /// Query the sum of the range [range.start, range.end)
        ///
        /// # Complexity
        /// - Time: O(log n)
        /// - Space: O(1)
        pub fn sum(&self, range: std::ops::Range<usize>) -> T
        where
            T: std::ops::Sub<Output = T>,
        {
            self.accum(range.end) - self.accum(range.start)
        }
    }
}

struct Solution {}

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        const MAX: usize = 100_000 + 1;

        let rating = rating.into_iter().map(|x| x as usize).collect::<Vec<_>>();

        let n = rating.len();
        let mut fenwick_pref = cm_fenwick::Fenwick::new(MAX);
        let mut fenwick_suf = cm_fenwick::Fenwick::new(MAX);

        for i in 0..n {
            fenwick_suf.add(rating[i], 1);
        }

        let mut ans = 0;
        for i in 0..n {
            fenwick_suf.add(rating[i], -1);

            let less_before = fenwick_pref.sum(0..rating[i]);
            let grea_after = fenwick_suf.sum((rating[i] + 1)..MAX);
            ans += less_before * grea_after;

            let grea_before = fenwick_pref.sum((rating[i] + 1)..MAX);
            let less_after = fenwick_suf.sum(0..rating[i]);
            ans += grea_before * less_after;

            fenwick_pref.add(rating[i], 1);
        }

        ans
    }
}
