// https://leetcode.com/problems/number-of-provinces

struct Solution {}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut used = vec![false; 201];
        let mut ans = 0;
        for i in 0..is_connected.len() {
            if !used[i] {
                ans += 1;
                Solution::dfs(i, &is_connected, &mut used);
            }
        }
        ans
    }

    fn dfs(cur: usize, is_connected: &Vec<Vec<i32>>, used: &mut Vec<bool>) {
        used[cur] = true;
        for (to, c) in is_connected[cur].iter().enumerate() {
            if !used[to] && c == &1 {
                Solution::dfs(to, is_connected, used);
            }
        }
    }
}
