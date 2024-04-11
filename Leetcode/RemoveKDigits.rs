// https://leetcode.com/problems/remove-k-digits

struct Solution {}

impl Solution {
    pub fn remove_kdigits(mut num: String, mut k: i32) -> String {
        let n = num.len();
        let num = num.as_bytes();

        let mut ans = vec![];
        let mut j = 0;

        ans.push(b'0');
        while j < n {
            if ans.last().unwrap() <= &num[j] {
                ans.push(num[j]);
                j += 1;
            } else if k > 0 {
                k -= 1;
                ans.pop();
            } else {
                while j < n {
                    ans.push(num[j]);
                    j += 1;
                }
            }
        }

        let ans_n = ans.len() - (k as usize).min(ans.len());
        let mut ans = std::str::from_utf8(&ans[1..ans_n])
            .unwrap()
            .trim_start_matches(|c: char| c == '0')
            .to_string();

        if ans.is_empty() {
            ans.push('0');
        }

        ans
    }
}
