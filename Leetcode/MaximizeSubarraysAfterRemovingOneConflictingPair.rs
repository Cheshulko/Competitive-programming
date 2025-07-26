// https://leetcode.com/problems/maximize-subarrays-after-removing-one-conflicting-pair

struct Solution {}

impl Solution {
    pub fn max_subarrays(n: i32, mut conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        use std::collections::BTreeSet;

        conflicting_pairs.push(vec![n, n + 1]);

        let mut conflicting_pairs = conflicting_pairs
            .into_iter()
            .map(|pair| {
                let a = pair[0] as usize - 1;
                let b = pair[1] as usize - 1;

                (a.min(b), a.max(b))
            })
            .collect::<Vec<_>>();

        conflicting_pairs.sort_unstable_by_key(|&(a, b)| (b, a));

        let mut starts = vec![];
        let mut ends = BTreeSet::new();

        for (i, &(a, b)) in conflicting_pairs.iter().enumerate() {
            starts.push((a, i));
            ends.insert((b, i));
        }

        starts.sort_unstable();
        starts.reverse();

        let n = n as usize;
        let m = conflicting_pairs.len();

        let mut boost = vec![0; m];
        let mut all = 0;
        for point in 0..n {
            while let Some(&(a, ai)) = starts.last() {
                if a < point {
                    starts.pop();
                    ends.remove(&(conflicting_pairs[ai].1, ai));
                } else {
                    break;
                }
            }

            let mut it = ends.range((point, usize::MAX)..);
            let &(cur_end, ind) = it.next().unwrap();

            all += cur_end - point;

            // not dummy last
            if let Some(&(next_end, _)) = it.next() {
                if next_end > cur_end {
                    boost[ind] += next_end - cur_end;
                }
            }
        }

        (all + boost.iter().max().unwrap_or(&0)) as i64
    }
}
