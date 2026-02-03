// https://leetcode.com/problems/trionic-array-i

struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let mut cnt = 1;
        let mut prev = nums[0];
        let mut inc = true;
        for &x in nums.iter().skip(1) {
            if prev == x {
                return false;
            }

            if inc {
                if prev > x {
                    cnt += 1;
                    inc = !inc;
                }
            } else {
                if prev < x {
                    cnt += 1;
                    inc = !inc;
                }
            }

            prev = x;
        }

        cnt == 3 && nums[0] < nums[1] && nums[n - 2] < nums[n - 1]
    }
}
