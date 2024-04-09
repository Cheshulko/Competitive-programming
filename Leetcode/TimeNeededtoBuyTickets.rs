// https://leetcode.com/problems/time-needed-to-buy-tickets

struct Solution {}

impl Solution {
    pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        let mut ans = 0;
        while tickets[k] != 0 {
            for i in 0..tickets.len() {
                if tickets[i] > 0 {
                    tickets[i] -= 1;
                    ans += 1;
                }

                if tickets[k] == 0 {
                    break;
                }
            }
        }

        ans
    }
}
