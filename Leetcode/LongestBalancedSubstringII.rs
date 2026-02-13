// https://leetcode.com/problems/longest-balanced-substring-ii

struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        use std::collections::HashMap;

        let s = s
            .into_bytes()
            .into_iter()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();

        let mut seen = HashMap::new();
        seen.insert((0, 0), -1);

        let mut seen_pair: [HashMap<_, _>; 3] = core::array::from_fn(|_| HashMap::new());
        for k in 0..3 {
            seen_pair[k].insert((0, 0), -1);
        }

        let mut seen_one: [HashMap<_, _>; 3] = core::array::from_fn(|_| HashMap::new());
        for k in 0..3 {
            seen_one[k].insert((0, 0), -1);
        }

        let mut cnt = [0; 3];
        let mut ans = 1;
        for (i, c) in s.into_iter().enumerate() {
            let i = i as i32;

            cnt[c] += 1;

            {
                let ba = cnt[1] - cnt[0];
                let ca = cnt[2] - cnt[0];

                if let Some(j) = seen.get(&(ba, ca)) {
                    ans = ans.max(i - j);
                }

                if !seen.contains_key(&(ba, ca)) {
                    seen.insert((ba, ca), i);
                }
            }

            {
                for k in 0..3 {
                    let key = (cnt[k], (cnt[(k + 2) % 3] - cnt[(k + 1) % 3]));
                    if let Some(j) = seen_pair[k].get(&key) {
                        ans = ans.max(i - j);
                    }
                    if !seen_pair[k].contains_key(&key) {
                        seen_pair[k].insert(key, i);
                    }
                }
            }

            {
                for k in 0..3 {
                    let key = (cnt[k], cnt[(k + 1) % 3]);
                    if let Some(j) = seen_one[k].get(&key) {
                        ans = ans.max(i - j);
                    }
                    if !seen_one[k].contains_key(&key) {
                        seen_one[k].insert(key, i);
                    }
                }
            }
        }

        ans
    }
}
