impl Solution {
    pub fn minimum_cost(mut nums: Vec<i32>) -> i32 {
        &nums[1..].sort();
        nums[0] + nums[1] + nums[2]
    }
}
