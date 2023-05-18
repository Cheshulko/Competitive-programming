// https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes

struct Solution {}

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        let mut ans = vec![];
        let mut used = vec![false; n];

        edges.into_iter().for_each(|v| {
            graph[v[0] as usize].push(v[1] as usize);
        });

        let topological_nodes = Solution::topological_sort(n, &graph);

        for node in topological_nodes.iter().rev() {
            if !used[*node] {
                ans.push(*node);
                Solution::dfs(*node, &graph, &mut used);
            }
        }

        ans.into_iter().map(|node| node as i32).collect()
    }

    fn dfs(cur: usize, edges: &Vec<Vec<usize>>, used: &mut Vec<bool>) {
        used[cur] = true;
        for to in &edges[cur] {
            if !used[*to] {
                Solution::dfs(*to, edges, used);
            }
        }
    }

    fn dfs_topological(
        cur: usize,
        edges: &Vec<Vec<usize>>,
        used: &mut Vec<bool>,
        ans: &mut Vec<usize>,
    ) {
        used[cur] = true;
        for to in &edges[cur] {
            if !used[*to] {
                Solution::dfs_topological(*to, edges, used, ans);
            }
        }
        ans.push(cur);
    }

    fn topological_sort(n: usize, edges: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut ans: Vec<usize> = vec![];
        let mut used = vec![false; n];

        for to in 0..n {
            if !used[to] {
                Solution::dfs_topological(to, edges, &mut used, &mut ans);
            }
        }

        ans
    }
}
