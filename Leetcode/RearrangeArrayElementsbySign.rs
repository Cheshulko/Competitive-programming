// https://leetcode.com/problems/rearrange-array-elements-by-sign

struct Solution {}

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![];
        let mut i = vec![0; 2];

        let go_next = |k: &mut usize, cmp: &(dyn Fn(&i32, &i32) -> bool)| {
            while *k < n && cmp(&nums[*k], &0) {
                *k += 1;
            }
        };

        let i_fn: Vec<&(dyn Fn(&i32, &i32) -> bool)> =
            vec![&core::cmp::PartialOrd::le, &core::cmp::PartialOrd::gt];

        go_next(&mut i[0], i_fn[0]);
        go_next(&mut i[1], i_fn[1]);

        let mut state = 0;

        while i[0] < n || i[1] < n {
            go_next(&mut i[state], i_fn[state]);
            if i[state] < n {
                ans.push(nums[i[state]]);
                i[state] += 1;
            }
            state ^= 1;
        }

        ans
    }
}
