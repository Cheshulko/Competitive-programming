impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let k = k as usize;

        let mut e = vec![];
        for i in 0..n {
            e.push((start_time[i], end_time[i]));
        }
        e.push((0, 0));
        e.push((event_time, event_time));
        e.sort_unstable();
        // n + 2

        let mut ma = 0;
        // n + 1
        let mut slots = vec![];
        for w in e.windows(2) {
            let (e1, e2) = (w[0], w[1]);
            let free = e2.0 - e1.1;

            ma = ma.max(free);

            slots.push(free);
        }

        let mut free = 0;
        let k = k + 1;
        for i in 0..k {
            free += slots[i];
        }
        ma = ma.max(free);

        for i in k..slots.len() {
            free -= slots[i - k];
            free += slots[i];
            ma = ma.max(free);
        }

        ma
    }
}
