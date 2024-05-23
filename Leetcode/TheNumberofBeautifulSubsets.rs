// https://leetcode.com/problems/the-number-of-beautiful-subsets

struct Solution {}

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ans = 0;

        for i in 1..(1 << n) {
            let mut x = vec![];
            for j in 0..n {
                if i & (1 << j) > 0 {
                    x.push(nums[j]);
                }
            }

            let mut ok = true;
            for j in 0..x.len() {
                for jj in (j + 1)..x.len() {
                    if (x[j] - x[jj]).abs() == k {
                        ok = false;
                        break;
                    }
                }
            }
            ans += ok as i32;
        }

        ans
    }
}
