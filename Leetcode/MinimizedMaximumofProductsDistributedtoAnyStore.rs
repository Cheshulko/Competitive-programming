// https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store

struct Solution {}

impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut l = 0;
        let mut r = 100_000 + 5;

        fn check(n: usize, quantities: &Vec<i32>, max: i32) -> bool {
            let mut cnt = 0;
            for &q in quantities.iter() {
                cnt += (q + max - 1) / max;
            }

            cnt as usize <= n
        }

        while r - l > 1 {
            let m = (l + r) / 2;
            if check(n, &quantities, m) {
                r = m;
            } else {
                l = m;
            }
        }

        r
    }
}
