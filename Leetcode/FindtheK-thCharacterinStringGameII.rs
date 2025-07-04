// https://leetcode.com/problems/find-the-k-th-character-in-string-game-ii

struct Solution {}

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        fn solve(k: i64, pow: usize, operations: &Vec<i32>) -> i32 {
            let len = 1 << pow;
            if len == 1 {
                return 0;
            }

            if k < len / 2 {
                return solve(k, pow - 1, operations);
            } else {
                return operations[pow - 1] + solve(k - len / 2, pow - 1, operations);
            }
        }

        let mut len = 1;
        let mut pow = 0;
        while len <= k {
            len *= 2;
            pow += 1;
        }

        assert!(pow <= operations.len() + 1);
        let ans = solve(k - 1, pow, &operations);

        (b'a' + (ans % 26) as u8) as char
    }
}
