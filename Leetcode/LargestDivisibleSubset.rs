// https://leetcode.com/problems/largest-divisible-subset

struct Solution {}

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut dp = vec![(-1, 0, 1); n];

        nums.sort();
        for i in 0..n {
            dp[i].1 = i;
        }
        for (i, x) in nums.iter().enumerate() {
            for (j, y) in nums.iter().enumerate().skip(i + 1) {
                if *y % *x == 0 && dp[j].2 < dp[i].2 + 1 {
                    dp[j].2 = dp[i].2 + 1;
                    dp[j].0 = i as i32;
                }
            }
        }

        let mut cur = dp.iter().max_by(|a, b| a.2.cmp(&b.2)).unwrap();
        let mut ans = vec![];

        loop {
            ans.push(nums[cur.1 as usize]);

            if cur.0 == -1 {
                ans.reverse();
                break ans;
            }

            cur = &dp[cur.0 as usize];
        }
    }
}
