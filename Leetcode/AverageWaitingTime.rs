// https://leetcode.com/problems/average-waiting-time

struct Solution {}

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let n = customers.len() as f64;
        let mut ans = 0.0;
        let mut cur = 0;

        for v in customers.into_iter() {
            let a = v[0];
            let w = v[1];

            let st = a.max(cur);
            cur = st + w;

            ans += (st - a + w) as f64;
        }

        ans / n
    }
}
