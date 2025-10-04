// https://leetcode.com/problems/container-with-most-water

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::collections::BTreeSet;

        let max_height = height.iter().max().copied().unwrap_or(0) as usize;

        let mut ordered = height
            .iter()
            .enumerate()
            .map(|(i, &h)| (i, h as usize))
            .collect::<BTreeSet<_>>();

        let by_height =
            height
                .into_iter()
                .enumerate()
                .fold(vec![vec![]; max_height + 1], |mut v, (i, h)| {
                    v[h as usize].push(i);
                    v
                });

        let mut ans = 0;
        for h in 0..=max_height {
            let left_most = ordered.iter().next();
            let right_most = ordered.iter().last();

            if let (Some((left, _)), Some((right, _))) = (left_most, right_most) {
                ans = ans.max(h * (right - left));
            }

            for &i in by_height[h].iter() {
                ordered.remove(&(i, h));
            }
        }

        ans as i32
    }
}
