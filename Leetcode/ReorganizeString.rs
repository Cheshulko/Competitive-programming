use std::cmp::Reverse;
use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut b = BTreeSet::<Reverse<(usize, char)>>::new();
        let mut cnt = vec![0; 26];

        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        for i in 0..26 {
            if cnt[i] > 0 {
                b.insert(Reverse((cnt[i], (i as u8 + b'a') as char)));
            }
        }

        let mut ans = vec![];

        while !b.is_empty() {
            let mut b_iter = b.iter();
            let mut cur = b_iter.next();
            if let (Some(last), Some(value)) = (ans.last(), cur) {
                if last == &value.0 .1 {
                    if b.len() < 2 {
                        return "".into();
                    } else {
                        cur = b_iter.next();
                    }
                }
            }

            let mut cur = *cur.take().unwrap();
            cur.0 .0 -= 1;

            ans.push(cur.0 .1);

            if cur.0 .0 > 0 {
                b.insert(cur);
            }
        }

        ans.iter().collect()
    }
}
