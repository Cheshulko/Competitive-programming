use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut bh = BinaryHeap::<(i32, char)>::new();
        if a > 0 {
            bh.push((a, 'a'));
        }
        if b > 0 {
            bh.push((b, 'b'));
        }
        if c > 0 {
            bh.push((c, 'c'));
        }

        let mut ans = vec![];
        while let Some((x, y)) = bh.pop() {
            if ans.len() > 1 && ans[ans.len() - 1] == y && ans[ans.len() - 2] == y {
                if let Some((x2, y2)) = bh.pop() {
                    bh.push((x, y));
                    ans.push(y2);

                    if x2 > 1 {
                        bh.push((x2 - 1, y2));
                    }
                } else {
                    break;
                }
            } else {
                ans.push(y);
                if x > 1 {
                    bh.push((x - 1, y));
                }
            }
        }

        ans.into_iter().collect()
    }
}
