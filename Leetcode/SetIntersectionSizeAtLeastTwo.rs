// https://leetcode.com/problems/set-intersection-size-at-least-two

struct Solution {}

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        use std::collections::BTreeSet;

        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut points = BTreeSet::<i32>::new();
        for interval in intervals {
            let li = interval[0];
            let mut ri = interval[1];

            let mut it = points.range(..=ri);
            let mut have = 0;
            while let Some(&v) = it.next_back() {
                if v < li || have == 2 {
                    break;
                }

                have += 1;
            }

            while have < 2 {
                while points.contains(&ri) {
                    ri -= 1;
                }
                points.insert(ri);
                have += 1;
            }
        }

        points.len() as i32
    }
}
