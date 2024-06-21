// https://leetcode.com/problems/grumpy-bookstore-owner

struct Solution {}

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let n = customers.len();
        let minutes = (minutes as usize).min(n);
        let mut pref_not_sat = vec![0; n + 1];
        let mut sum = 0;

        for i in 0..n {
            sum += if grumpy[i] == 1 { 0 } else { customers[i] };
            pref_not_sat[i + 1] = pref_not_sat[i] + if grumpy[i] == 0 { 0 } else { customers[i] };
        }

        let mut ans = sum;
        for i in 0..=(n - minutes) {
            ans = ans.max(sum + pref_not_sat[i + minutes] - pref_not_sat[i]);
        }

        ans
    }
}
