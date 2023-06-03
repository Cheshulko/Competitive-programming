// https://leetcode.com/problems/time-needed-to-inform-all-employees

struct Solution {}

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];

        for (i, x) in manager.iter().enumerate() {
            if *x != -1 {
                g[*x as usize].push(i);
            }
        }

        Solution::dfs(head_id as usize, &g, &inform_time)
    }

    fn dfs(cur: usize, g: &Vec<Vec<usize>>, t: &Vec<i32>) -> i32 {
        let mut l = 0;
        for to in &g[cur] {
            l = l.max(t[cur] + Solution::dfs(*to, g, t));
        }
        l
    }
}
