// https://leetcode.com/problems/modify-graph-edge-weights

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    fn dijkstra(adj: &Vec<Vec<(usize, i64)>>, s: usize) -> Vec<i64> {
        let mut dist = vec![i64::MAX; adj.len()];
        dist[s] = 0;

        let mut visited = vec![false; adj.len()];
        let mut queue = BinaryHeap::new();

        queue.push(Reverse((0, s)));
        while let Some(Reverse((d, u))) = queue.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            for &(v, w) in &adj[u] {
                if dist[v] > d + w {
                    dist[v] = d + w;
                    queue.push(Reverse((dist[v], v)));
                }
            }
        }
        dist
    }

    pub fn modified_graph_edges(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let n = n as usize;
        let source = source as usize;
        let destination = destination as usize;
        let target = target as i64;
        let mut adj = vec![vec![]; n];
        let mut can_edges = vec![];
        let mut can = vec![vec![false; n]; n];
        for edge in edges.into_iter() {
            let v = edge[0] as usize;
            let u = edge[1] as usize;
            let mut w = edge[2] as i64;

            if w == -1 {
                can[v][u] = true;
                can[u][v] = true;
                can_edges.push((v, u, adj[v].len(), adj[u].len()));
                w = 1;
            }
            adj[v].push((u, w));
            adj[u].push((v, w));
        }

        let dist = Solution::dijkstra(&adj, source);
        let cur_dist = dist[destination];
        if cur_dist > target {
            return vec![];
        }

        if cur_dist == target {
            let mut ans = vec![];
            for i in 0..n {
                for &(to, w) in adj[i].iter() {
                    if to < i {
                        continue;
                    }
                    ans.push(vec![i as i32, to as i32, w as i32])
                }
            }
            return ans;
        }

        let over = target + 1 - cur_dist;
        let mut found = false;
        'out: for &(v, u, iv, iu) in can_edges.iter() {
            adj[v][iv] = (u, 1 + over);
            adj[u][iu] = (v, 1 + over);

            let dist = Solution::dijkstra(&adj, source);
            if dist[destination] >= target {
                let mut l = -1;
                let mut r = over + 1;

                found = true;
                while r - l > 1 {
                    let m = (l + r) / 2;
                    adj[v][iv] = (u, m);
                    adj[u][iu] = (v, m);

                    if Solution::dijkstra(&adj, source)[destination] <= target {
                        l = m;
                    } else {
                        r = m;
                    }

                    adj[v][iv] = (u, l);
                    adj[u][iu] = (v, l);
                }

                break 'out;
            }
        }

        if !found {
            return vec![];
        }

        let mut ans = vec![];
        for i in 0..n {
            for &(to, w) in adj[i].iter() {
                if to < i {
                    continue;
                }
                ans.push(vec![i as i32, to as i32, w as i32])
            }
        }

        ans
    }
}
