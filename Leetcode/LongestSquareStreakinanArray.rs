// https://leetcode.com/problems/longest-square-streak-in-an-array

struct Solution {}

impl Solution {
    pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        const MAX: usize = 100_000;

        let mut ans = 0;
        let mut have = vec![false; MAX + 1];
        let mut cnt = vec![0; MAX + 1];
        for x in nums.into_iter() {
            let x = x as usize;
            if have[x] {
                continue;
            }
            have[x] = true;
            cnt[x] += 1;
            if x * x <= MAX {
                cnt[x * x] += cnt[x];
            }
            ans = ans.max(cnt[x]);
        }

        if ans == 1 {
            -1
        } else {
            ans as i32
        }
    }
}
