// https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations

struct Solution {}

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        use std::collections::BTreeSet;

        fn go(mem: &mut BTreeSet<Vec<usize>>, cur: Vec<usize>, a: usize, b: usize) {
            if mem.contains(&cur) {
                return;
            }

            mem.insert(cur.clone());
            let n = cur.len();

            let mut v1 = cur.clone();
            for i in (1..n).step_by(2) {
                v1[i] += a;
                v1[i] %= 10;
            }
            go(mem, v1, a, b);

            let mut v2 = vec![0; n];
            for i in 0..n {
                v2[i] = cur[(i + n - b) % n];
            }
            go(mem, v2, a, b);
        }

        let s = s
            .into_bytes()
            .into_iter()
            .map(|b| (b - b'0') as usize)
            .collect::<Vec<_>>();

        let mut mem = BTreeSet::new();
        go(&mut mem, s, a as usize, b as usize);

        let ans = mem.first().unwrap();
        ans.iter()
            .map(|&b| (b as u8 + b'0') as char)
            .collect::<String>()
    }
}
