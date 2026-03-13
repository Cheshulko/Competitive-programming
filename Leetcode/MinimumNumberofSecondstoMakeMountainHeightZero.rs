// https://leetcode.com/problems/minimum-number-of-seconds-to-make-mountain-height-zero

struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        use std::collections::BTreeSet;

        let mut mountain_height = mountain_height as i64;
        let mut worker_times = worker_times
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x as i64, x as i64, i as i64, 1))
            .collect::<BTreeSet<_>>();

        let mut time = 0;
        while let Some(worker) = worker_times.pop_first() {
            time = time.max(worker.0);
            worker_times.insert((
                worker.0 + worker.1 * (worker.3 + 1),
                worker.1,
                worker.2,
                (worker.3 + 1),
            ));

            mountain_height -= 1;
            if mountain_height == 0 {
                break;
            }
        }

        time
    }
}
