// https://leetcode.com/problems/magnetic-force-between-two-balls

struct Solution {}

impl Solution {
    fn check(d: i32, position: &Vec<i32>, mut m: i32) -> bool {
        let n = position.len();
        let mut cur = position[0];
        let mut i = 0;

        m -= 1;
        while i < n && m > 0 {
            while i < n && position[i] - cur < d {
                i += 1;
            }
            if i >= n {
                break;
            } else {
                cur = position[i];
                m -= 1
            }
        }

        m == 0
    }

    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        let n = position.len();
        position.sort_unstable();

        let d = (position[n - 1] - position[0]) / (m - 1);
        let mut ans = i32::MAX;

        let mut l = 0;
        let mut r = d + 1;

        while r - l > 1 {
            let mid = (l + r) / 2;
            let c = Solution::check(mid, &position, m);
            if c {
                l = mid;
            } else {
                r = mid;
            }
        }

        l
    }
}
