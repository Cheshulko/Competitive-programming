// https://leetcode.com/problems/count-good-triplets-in-an-array

mod cm_fenwick {
    // [l; r)
    pub struct Fenwick<T> {
        ary: Vec<T>,
    }

    impl<T: Clone + Default + std::ops::AddAssign<T>> Fenwick<T> {
        /// - Time: O(n)
        /// - Space: O(n)
        pub fn new(n: usize) -> Self {
            Fenwick {
                ary: vec![T::default(); n],
            }
        }

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

        // O(log n)
        pub fn add(&mut self, mut idx: usize, val: T) {
            while idx < self.ary.len() {
                self.ary[idx] += val.clone();
                idx |= idx + 1;
            }
        }

        /// [range.start, range.end). O(log n)
        pub fn sum(&self, range: std::ops::Range<usize>) -> T
        where
            T: std::ops::Sub<Output = T>,
        {
            self.accum(range.end) - self.accum(range.start)
        }
    }
}

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();

        let mut index_in_2 = vec![0; n];
        for (i, &x) in nums2.iter().enumerate() {
            index_in_2[x as usize] = i;
        }

        let mut f1 = cm_fenwick::Fenwick::new(n + 1);
        let mut pairs = vec![0; n];
        for i in (0..n).rev() {
            let x1 = nums1[i] as usize;
            let ind2 = index_in_2[x1];
            let v = f1.sum(ind2 + 1..n + 1);
            pairs[x1] = v;

            f1.add(ind2, 1);
        }

        let mut ans = 0;
        let mut f1_ = cm_fenwick::Fenwick::new(n + 1);
        for i in (0..n).rev() {
            let x1 = nums1[i] as usize;
            let ind2 = index_in_2[x1];
            let v = f1_.sum(ind2 + 1..n + 1);
            ans += v;

            f1_.add(ind2, pairs[x1]);
        }

        ans
    }
}
