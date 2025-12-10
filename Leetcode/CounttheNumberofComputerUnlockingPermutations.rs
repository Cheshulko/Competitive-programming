// https://leetcode.com/problems/count-the-number-of-computer-unlocking-permutations

struct Solution {}

impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        let mi = complexity.iter().min().copied().unwrap();
        if mi != complexity[0] {
            return 0;
        }
        if complexity.iter().filter(|&&c| c == mi).count() > 1 {
            return 0;
        }

        const M: usize = 1_000_000_000 + 7;

        let n = complexity.len();

        let mut ans = 1;
        for i in 1..=n - 1 {
            ans *= i;
            ans %= M;
        }

        ans as i32
    }
}
