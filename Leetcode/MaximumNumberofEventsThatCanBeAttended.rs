// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended

use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events
            .into_iter()
            .map(|event| (event[0] as usize, event[1] as usize))
            .collect::<Vec<_>>();

        events.sort_unstable();

        let mi = events.iter().min_by_key(|event| event.0).unwrap().0;
        let ma = events.iter().max_by_key(|event| event.1).unwrap().1;

        let mut begins = vec![vec![]; ma + 1];
        let mut ends = vec![vec![]; ma + 2];

        for (i, event) in events.iter().enumerate() {
            begins[event.0].push(i);
            ends[event.1 + 1].push(i);
        }

        let mut in_ends = BTreeSet::new();
        let mut ans = 0;
        for cur in mi..=ma {
            for &i in begins[cur].iter() {
                in_ends.insert((events[i].1, i));
            }

            for &i in ends[cur].iter() {
                in_ends.remove(&(events[i].1, i));
            }

            if let Some(take) = in_ends.pop_first() {
                ans += 1;
                in_ends.remove(&take);
            }
        }

        ans
    }
}
