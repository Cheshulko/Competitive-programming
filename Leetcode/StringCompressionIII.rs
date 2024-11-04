// https://leetcode.com/problems/string-compression-iii

struct Solution {}

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut ans = vec![];

        let word = word.into_bytes();
        let n = word.len();

        let mut i = 0;
        let mut cnt = 0;
        while i < n {
            let cur = word[i];
            while i < n && cnt < 9 && word[i] == cur {
                cnt += 1;
                i += 1;
            }
            ans.push(b'0' + cnt);
            ans.push(cur);

            cnt = 0;
        }

        String::from_utf8(ans).unwrap()
    }
}
