// https://leetcode.com/problems/final-value-of-variable-after-performing-operations

struct Solution {}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.into_iter().fold(0, |ans, op| {
            let mut op = op.into_bytes();
            op.sort_unstable();

            match op[0] {
                b'+' => ans + 1,
                b'-' => ans - 1,
                _ => unreachable!(),
            }
        })
    }
}
