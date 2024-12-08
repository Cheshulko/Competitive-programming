// https://leetcode.com/problems/two-best-non-overlapping-events

struct Solution {}

impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        let n = events.len();

        events.sort_by_key(|event| event[1]);

        let mut prefix = vec![0; n + 1];
        for i in 1..=n {
            prefix[i] = prefix[i - 1].max(events[i - 1][2]);
        }

        let mut ans = 0;
        for current in events.iter() {
            let pos = events.partition_point(|event| event[1] < current[0]);
            ans = ans.max(current[2] + prefix[pos]);
        }

        ans
    }
}
