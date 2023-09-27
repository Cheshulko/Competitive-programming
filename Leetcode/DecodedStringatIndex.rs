// https://leetcode.com/problems/decoded-string-at-index

struct Solution {}

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut k = k as i64 - 1;
        let mut cur_len = 0;
        let mut q = vec![];

        for c in s.chars() {
            q.push(c);

            if let Some(d) = c.to_digit(10) {
                cur_len *= d as i64;
            } else {
                cur_len += 1;
            }

            if k < cur_len {
                let ans = loop {
                    if k == 0 {
                        break q[0];
                    }

                    let last = q.pop().unwrap();
                    if let Some(d) = last.to_digit(10) {
                        cur_len /= d as i64;

                        if cur_len - 1 == k {
                            break q.pop().unwrap();
                        }

                        k %= cur_len;
                    } else {
                        if cur_len - 1 == k {
                            break last;
                        }

                        cur_len -= 1;
                    }
                };

                return format!("{}", ans);
            }
        }
        unreachable!();
    }
}
