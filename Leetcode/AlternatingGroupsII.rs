// https://leetcode.com/problems/alternating-groups-ii

struct Solution {}

impl Solution {
    pub fn number_of_alternating_groups(mut colors: Vec<i32>, k: i32) -> i32 {
        let k = k as usize - 1;
        let n = colors.len();

        colors.push(colors[0]);
        let colors = colors
            .windows(2)
            .map(|v| (v[0] != v[1]) as usize)
            .collect::<Vec<_>>();

        let mut cnt = 0;
        let mut cur = colors.iter().take(k).sum::<usize>();

        for i in k..n + k {
            cnt += (cur == k) as usize;

            cur -= colors[i - k];
            cur += colors[i % n];
        }

        cnt as i32
    }
}
