// https://leetcode.com/problems/minimum-penalty-for-a-shop

struct Solution {}

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut customers = customers.chars().map(|c| c as u8).collect::<Vec<_>>();
        customers.push(0);
        let n = customers.len();

        let mut pref_N = vec![0; n + 1];
        let mut suf_Y = vec![0; n + 1];

        for i in 0..n {
            pref_N[i + 1] = pref_N[i] + (customers[i] == b'N') as i32;
        }

        for i in (0..n).rev() {
            suf_Y[i] = suf_Y[i + 1] + (customers[i] == b'Y') as i32;
        }

        let mut ind = usize::MAX;
        let mut ans = i32::MAX;

        for i in 0..n {
            let cur = pref_N[i] + suf_Y[i];
            if cur < ans {
                ind = i;
                ans = cur;
            }
        }

        ind as i32
    }
}
