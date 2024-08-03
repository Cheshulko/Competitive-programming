// https://leetcode.com/problems/time-taken-to-mark-all-nodes

use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    fn get_dist(node: usize) -> i32 {
        if node % 2 == 1 {
            1
        } else {
            2
        }
    }

    fn dfs_dist_from_leafs(cur: usize, p: i32, adj: &Vec<Vec<usize>>, dist: &mut Vec<i32>) -> i32 {
        for &to in adj[cur].iter() {
            if to as i32 == p {
                continue;
            }

            dist[cur] = dist[cur].max(
                Solution::dfs_dist_from_leafs(to, cur as i32, adj, dist) + Solution::get_dist(to),
            );
        }

        dist[cur]
    }

    fn dfs_for_node(
        cur: usize,
        p: i32,
        p_dist: i32,
        adj: &Vec<Vec<usize>>,
        dist: &Vec<i32>,
        par_dist: &mut Vec<i32>,
    ) {
        par_dist[cur] = p_dist;

        let mut max_children_dists = BinaryHeap::<(i32, i32)>::new();
        max_children_dists.push((p_dist, p));
        for &to in adj[cur].iter() {
            if to as i32 == p {
                continue;
            }

            max_children_dists.push((dist[to] + Solution::get_dist(to), to as i32));
        }

        if max_children_dists.len() == 1 {
            return;
        }

        let max1 = max_children_dists.pop().unwrap();
        let max2 = max_children_dists.pop().unwrap();

        par_dist[cur] = par_dist[cur].max(max1.0);

        for &to in adj[cur].iter() {
            if to as i32 == p {
                continue;
            }

            let mut lmax = max1.0;
            if to as i32 == max1.1 {
                lmax = max2.0;
            }

            Solution::dfs_for_node(
                to,
                cur as i32,
                lmax + Solution::get_dist(cur),
                adj,
                dist,
                par_dist,
            );
        }
    }

    pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut n = 0;
        for x in edges.iter() {
            n = n.max(x[0] as usize).max(x[1] as usize);
        }
        n += 1;

        let mut adj = vec![vec![]; n];
        for x in edges.into_iter() {
            adj[x[0] as usize].push(x[1] as usize);
            adj[x[1] as usize].push(x[0] as usize);
        }

        let mut dist = vec![0; n];
        Solution::dfs_dist_from_leafs(0, -1, &adj, &mut dist);

        let mut par_dist = vec![0; n];
        Solution::dfs_for_node(0, -1, 0, &adj, &dist, &mut par_dist);

        par_dist
    }
}
