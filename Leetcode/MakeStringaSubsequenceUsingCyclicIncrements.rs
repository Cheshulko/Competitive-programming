//  https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments

struct Solution {}

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let str1 = str1.into_bytes();
        let str2 = str2.into_bytes();

        let mut j = 0;
        for b in str1.into_iter() {
            if str2[j] == b || (1 + b - b'a') % (1 + b'z' - b'a') + b'a' == str2[j] {
                j += 1
            }

            if j == str2.len() {
                return true;
            }
        }

        return false;
    }
}
