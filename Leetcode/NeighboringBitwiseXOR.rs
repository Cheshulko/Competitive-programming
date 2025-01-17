// https://leetcode.com/problems/neighboring-bitwise-xor

struct Solution {}

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let n = derived.len();

        let mut ans = [0; 2];
        for (i, &x) in derived.iter().enumerate() {
            ans[(i + 1) % n % 2] = ans[i % 2] ^ x;
        }

        ans[0] == 0
    }
}
