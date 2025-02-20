// https://leetcode.com/problems/find-unique-binary-string

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        fn mex(input: Vec<usize>) -> usize {
            let hs = input.into_iter().collect::<HashSet<_>>();

            let mut ans = 0;
            while hs.contains(&&ans) {
                ans += 1;
            }

            ans
        }

        let n = nums.len();
        let mut v = vec![0];
        for num in nums.into_iter() {
            let num = num
                .into_bytes()
                .into_iter()
                .fold(0, |x, c| (x << 1) + (c - b'0') as usize);

            v.push(num);
        }

        let mut x = mex(v);
        let mut ans = vec![];
        for _ in 0..n {
            ans.push((x & 1) as u8 + b'0');
            x >>= 1;
        }
        ans.reverse();

        String::from_utf8(ans).unwrap()
    }
}
