// https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, s: usize, dt: usize) -> Vec<usize> {
        let mut dist = vec![usize::MAX; adj.len()];
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
                if dist[v] > d + w && d + w <= dt {
                    dist[v] = d + w;
                    queue.push(Reverse((dist[v], v)));
                }
            }
        }
        dist
    }
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let distance_threshold = distance_threshold as usize;

        let mut adj = vec![vec![]; n];
        for edge in edges.into_iter() {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            let d = edge[2] as usize;

            adj[x].push((y, d));
            adj[y].push((x, d));
        }

        let mut cnt = vec![0; n];
        for i in 0..n {
            let d = Solution::dijkstra(&adj, i, distance_threshold);
            cnt[i] = d.into_iter().filter(|&x| x != usize::MAX).count();
        }

        let mut ans = 0;
        let mut mn = usize::MAX;
        for i in 0..n {
            if cnt[i] <= mn {
                mn = cnt[i];
                ans = i;
            }
        }

        ans as i32
    }
}
