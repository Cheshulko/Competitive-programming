// https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-strings

struct Solution {}

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut s = s
            .into_bytes()
            .into_iter()
            .map(|b| (b - b'a') as usize)
            .rev()
            .collect::<Vec<_>>();

        let mut cnt = vec![0; 26];
        for &c in s.iter() {
            cnt[c] += 1;
        }

        let mut t = vec![];
        let mut ans = vec![];
        for c in 0..26 {
            loop {
                if !t.is_empty() && *t.last().unwrap() <= c {
                    let ct = t.pop().unwrap();
                    ans.push(ct);
                } else if cnt[c] > 0 {
                    let cs = s.pop().unwrap();
                    assert!(cs >= c);
                    if cs == c {
                        ans.push(cs);
                    } else {
                        t.push(cs);
                    }
                    cnt[cs] -= 1;
                } else {
                    break;
                }
            }
        }

        while let Some(ct) = t.pop() {
            ans.push(ct);
        }

        ans.into_iter().map(|c| (c as u8 + b'a') as char).collect()
    }
}
