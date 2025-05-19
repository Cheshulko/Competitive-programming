// https://leetcode.com/problems/type-of-triangle

struct Solution {}

impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        nums.sort_unstable();

        let c1 = (nums[0] == nums[1]) as usize;
        let c2 = (nums[1] == nums[2]) as usize;
        let c3 = (nums[0] == nums[2]) as usize;

        if nums[0] + nums[1] <= nums[2] {
            "none".into()
        } else if c1 + c2 + c3 == 1 {
            "isosceles".into()
        } else if c1 + c2 + c3 == 3 {
            "equilateral".into()
        } else {
            "scalene".into()
        }
    }
}
