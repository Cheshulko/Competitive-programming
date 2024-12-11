// https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation

struct Solution {}

impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut begin = nums.into_iter().map(|num| num - k).collect::<Vec<_>>();
        begin.sort_unstable();

        let mut ans = 0;
        for (ind, num) in begin.iter().enumerate() {
            ans = ans.max(begin[ind..].partition_point(|&b| b < num + 2 * k + 1));
        }

        return ans as i32;
    }
}
