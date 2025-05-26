// https://leetcode.com/problems/largest-color-value-in-a-directed-graph

pub struct DirectedGraph {
    edges: Vec<Vec<usize>>,
    colors: Vec<usize>,
}

impl DirectedGraph {
    const ALPHABET: usize = 26;

    fn new(edges: Vec<Vec<i32>>, colors: String) -> DirectedGraph {
        let nodes = colors.len();

        let edges = edges
            .into_iter()
            .fold(vec![vec![]; nodes], |mut edges, edge| {
                let v = edge[0] as usize;
                let u = edge[1] as usize;
                edges[v].push(u);

                edges
            });

        let colors = colors
            .into_bytes()
            .into_iter()
            .map(|b| (b - b'a') as usize)
            .collect();

        return DirectedGraph { edges, colors };
    }

    fn find_starts(&self) -> Vec<usize> {
        let mut edges_in = vec![0; self.colors.len()];
        for (node_from, edges) in self.edges.iter().enumerate() {
            for &node_to in edges {
                if node_from == node_to {
                    return vec![];
                }

                edges_in[node_to] += 1;
            }
        }

        let starts = edges_in
            .into_iter()
            .enumerate()
            .filter_map(|(node, edges_cnt)| (edges_cnt == 0).then_some(node))
            .collect();

        return starts;
    }

    fn has_cycle(&self, node: usize, visited: &mut Vec<bool>, recStack: &mut Vec<bool>) -> bool {
        visited[node] = true;
        recStack[node] = true;

        for &node_to in self.edges[node].iter() {
            if recStack[node_to] {
                return true;
            }

            if !visited[node_to] && self.has_cycle(node_to, visited, recStack) {
                return true;
            }
        }

        recStack[node] = false;

        return false;
    }

    fn dfs(
        &self,
        parent: Option<usize>,
        cur: usize,
        paths: &mut Vec<Vec<i32>>,
        parents: &mut Vec<Option<usize>>,
    ) {
        let color = self.colors[cur];

        if self.edges[cur].len() == 0 {
            paths[cur][color] = 1;
        }

        parents[cur] = parent;

        for &node_to in self.edges[cur].iter() {
            if parents[node_to].is_none() {
                self.dfs(Some(cur), node_to, paths, parents);
            }

            for color_to in 0..DirectedGraph::ALPHABET {
                paths[cur][color_to] = std::cmp::max(
                    paths[cur][color_to],
                    paths[node_to][color_to] + (color_to == color) as i32,
                );
            }
        }
    }

    fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let nodes = colors.len();
        let graph = DirectedGraph::new(edges, colors);

        {
            let mut visited = vec![false; nodes];
            let mut recStack = vec![false; nodes];

            for node in 0..nodes {
                if !visited[node] {
                    if graph.has_cycle(node, &mut visited, &mut recStack) {
                        return -1;
                    }
                }
            }
        }

        let starts = graph.find_starts();
        if starts.len() <= 0 {
            return -1;
        }

        let mut paths = vec![vec![0; DirectedGraph::ALPHABET]; nodes];
        let mut parents = vec![Option::None; nodes];
        let mut ans = 0;
        for start in starts {
            graph.dfs(Option::None, start as usize, &mut paths, &mut parents);

            for lcolor in 0..DirectedGraph::ALPHABET {
                ans = std::cmp::max(ans, paths[start][lcolor]);
            }
        }
        return ans;
    }
}

struct Solution {}

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        DirectedGraph::largest_path_value(colors, edges)
    }
}
