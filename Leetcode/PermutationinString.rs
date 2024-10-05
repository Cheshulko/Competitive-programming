// https://leetcode.com/problems/permutation-in-string

struct Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();
        let s1n = s1.len();
        let s2n = s2.len();

        if s2n < s1n {
            return false;
        }

        let cnt1 = s1.into_iter().fold(vec![0; 26], |mut v, c| {
            v[(c - b'a') as usize] += 1;
            v
        });

        let mut cnt2 = vec![0; 26];
        for i in 0..s1n - 1 {
            cnt2[(s2[i] - b'a') as usize] += 1;
        }

        let check =
            |cnt2: &Vec<usize>| -> bool { cnt1.iter().zip(cnt2.iter()).all(|(a, b)| a == b) };

        for i in s1n - 1..s2n {
            cnt2[(s2[i] - b'a') as usize] += 1;

            if check(&cnt2) {
                return true;
            }

            cnt2[(s2[i - (s1n - 1)] - b'a') as usize] -= 1;
        }

        return false;
    }
}
