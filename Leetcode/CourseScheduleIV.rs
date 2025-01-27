// https://leetcode.com/problems/course-schedule-iv

struct Solution {}

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = num_courses as usize;

        let mut adj = vec![vec![]; n];
        for v in prerequisites.into_iter() {
            let u = v[0] as usize;
            let v = v[1] as usize;
            adj[u].push(v);
        }

        fn dfs(cur: usize, t: usize, adj: &Vec<Vec<usize>>, used: &mut Vec<bool>) -> bool {
            used[cur] = true;
            if cur == t {
                return true;
            }

            let mut res = false;
            for &to in adj[cur].iter() {
                if !used[to] {
                    res |= dfs(to, t, adj, used);
                }
            }

            res
        }

        queries
            .into_iter()
            .map(|v| {
                let u = v[0] as usize;
                let v = v[1] as usize;

                let mut used = vec![false; n];
                dfs(u, v, &adj, &mut used)
            })
            .collect()
    }
}
