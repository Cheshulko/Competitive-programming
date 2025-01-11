// https://leetcode.com/problems/construct-k-palindrome-strings

struct Solution {}

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let n = s.len();
        let k = k as usize;
        let count = s
            .into_bytes()
            .into_iter()
            .fold(vec![0; 26], |mut count, c| {
                count[(c - b'a') as usize] += 1;
                count
            });

        let odd = count.into_iter().filter(|count| count & 1 == 1).count();

        n >= k && odd <= k
    }
}
