// https://leetcode.com/problems/minimum-time-to-make-rope-colorful

struct Solution {}

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors = colors.as_bytes();
        let mut cur_ind = 0;
        let mut cur = 0;
        let mut ans = 0;

        while cur_ind < colors.len() {
            let mut sum = 0;
            let mut mx = i32::MIN;
            cur = colors[cur_ind];
            while cur_ind < colors.len() && cur == colors[cur_ind] {
                sum += needed_time[cur_ind];
                mx = mx.max(needed_time[cur_ind]);
                cur_ind += 1;
            }

            ans += sum - mx;
        }

        ans
    }
}
