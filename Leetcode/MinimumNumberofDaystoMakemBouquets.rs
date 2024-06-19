// https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets

struct Solution {}

impl Solution {
    fn check(bloom_day: &Vec<i32>, m: i32, k: i32, d: i32) -> bool {
        let n = bloom_day.len();
        let mut i = 0;
        let mut cur = 0;
        let mut ans = 0;

        while i < n {
            while i < n && bloom_day[i] <= d && cur < k {
                i += 1;
                cur += 1;
            }

            if cur == k {
                ans += 1;
            } else {
                i += 1;
            }
            cur = 0;
        }

        ans >= m
    }

    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let need = (m * k) as usize;
        if need > bloom_day.len() {
            return -1;
        }

        let mut l = 0;
        let mut r = *bloom_day.iter().max().unwrap();

        while r - l > 1 {
            let mid = (l + r) / 2;
            if Solution::check(&bloom_day, m, k, mid) {
                r = mid;
            } else {
                l = mid;
            }
        }

        r
    }
}
