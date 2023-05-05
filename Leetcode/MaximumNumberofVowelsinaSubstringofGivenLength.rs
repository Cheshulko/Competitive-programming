// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length

struct Solution {}

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let vowel = ['a', 'e', 'i', 'o', 'u'];
        let k = k as usize;
        let v = s.chars().collect::<Vec<char>>();
        let mut shift = 0;
        let mut cnt = 0;
        let mut ans = 0;

        ans = s[0 + shift..k + shift]
            .chars()
            .filter(|c| vowel.contains(&c))
            .count();
        cnt = ans;
        shift += 1;

        while k + shift - 1 < s.len() {
            cnt -= vowel.contains(&v[shift - 1]) as usize;
            cnt += vowel.contains(&v[k + shift - 1]) as usize;
            shift += 1;
            ans = ans.max(cnt);
        }

        ans as i32
    }
}
