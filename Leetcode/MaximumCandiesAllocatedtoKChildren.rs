// https://leetcode.com/problems/maximum-candies-allocated-to-k-children

struct Solution {}

impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let k = k as usize;

        fn can(candies: &Vec<i32>, k: usize, x: i32) -> bool {
            let mut cnt = 0;

            for &candy in candies.iter() {
                cnt += (candy / x) as usize;
            }

            cnt >= k
        }

        let mut l = 0;
        let mut r = candies.iter().max().unwrap() + 1;

        while r - l > 1 {
            let m = (l + r) >> 1;
            if can(&candies, k, m) {
                l = m;
            } else {
                r = m;
            }
        }

        l
    }
}
