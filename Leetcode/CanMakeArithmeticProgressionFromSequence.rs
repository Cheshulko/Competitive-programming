// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence

struct Solution {}

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let d = arr[1] - arr[0];
        arr.iter()
            .enumerate()
            .skip(1)
            .all(|(i, x)| x - arr[i - 1] == d)
    }
}
