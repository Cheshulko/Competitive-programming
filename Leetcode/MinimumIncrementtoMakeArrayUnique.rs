// https://leetcode.com/problems/minimum-increment-to-make-array-unique

struct Solution {}

impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        const MAX: usize = 2 * 100_000 + 1;
        let mut cnt = vec![0; MAX];
        for x in nums.into_iter() {
            cnt[x as usize] += 1;
        }

        let mut ans = 0;
        for x in 0..MAX {
            if cnt[x] > 1 {
                ans += cnt[x] - 1;
                cnt[x + 1] += cnt[x] - 1;
            }
        }

        ans
    }
}
