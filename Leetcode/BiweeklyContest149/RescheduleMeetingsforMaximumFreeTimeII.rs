use std::collections::BTreeSet;

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();

        let mut e = vec![];
        for i in 0..n {
            e.push((start_time[i], end_time[i]));
        }
        e.push((0, 0));
        e.push((event_time, event_time));
        e.sort_unstable();

        let mut ma = 0;
        let mut slots = BTreeSet::new();
        for w in e.windows(2) {
            let (e1, e2) = (w[0], w[1]);
            let free = e2.0 - e1.1;

            ma = ma.max(free);

            slots.insert((free, e1.1, e2.0));
        }

        for i in 1..=n {
            let cur_e = e[i];
            let prev_e = e[i - 1];
            let next_e = e[i + 1];

            let free1 = cur_e.0 - prev_e.1;
            let free2 = next_e.0 - cur_e.1;

            ma = ma.max(free1 + free2);

            slots.remove(&(free1, prev_e.1, cur_e.0));
            slots.remove(&(free2, cur_e.1, next_e.0));

            if let Some(last) = slots.last() {
                if last.0 >= cur_e.1 - cur_e.0 {
                    ma = ma.max(free1 + free2 + cur_e.1 - cur_e.0);
                }
            }

            slots.insert((free1, prev_e.1, cur_e.0));
            slots.insert((free2, cur_e.1, next_e.0));
        }

        ma
    }
}
