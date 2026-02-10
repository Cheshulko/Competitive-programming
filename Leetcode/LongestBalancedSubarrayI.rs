// https://leetcode.com/problems/longest-balanced-subarray-i

struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        const M: usize = 100_000 + 1;

        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();

        let mut seen = [0; M];
        let mut even = 0;
        let mut odd = 0;
        let mut ans = 0;
        for i in 0..nums.len() {
            let num = nums[i];
            seen[num] += 1;

            if seen[num] == 1 {
                odd += num & 1;
                even += 1 - num & 1;
            }
            if odd == even {
                ans = ans.max(i + 1);
            }

            let mut odd_range = odd;
            let mut even_range = even;
            let mut seen_pref = [0; M];
            for j in 0..i {
                let num = nums[j];
                seen_pref[num] += 1;

                if seen_pref[num] == seen[num] {
                    odd_range -= num & 1;
                    even_range -= 1 - num & 1;
                }

                if even_range == odd_range {
                    ans = ans.max(i - j);
                }
            }
        }

        ans as i32
    }
}
