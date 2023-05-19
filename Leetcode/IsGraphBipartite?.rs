// https://leetcode.com/problems/is-graph-bipartite

struct Solution {}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut colors = vec![-1; n];
        let mut res = true;

        for i in 0..n {
            if colors[i] == -1 {
                res &= Solution::dfs(i, &graph, 0, &mut colors);
            }
        }

        res
    }

    fn dfs(cur: usize, graph: &Vec<Vec<i32>>, color: i32, colors: &mut Vec<i32>) -> bool {
        colors[cur] = color;
        let mut res = true;
        for i in &graph[cur] {
            let to = *i as usize;
            if colors[to] != -1 {
                res &= colors[to] != color
            } else {
                res &= Solution::dfs(to, graph, 1 - color, colors);
            }
        }

        res
    }
}
