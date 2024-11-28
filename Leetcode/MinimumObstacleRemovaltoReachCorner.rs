// https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    const DIRS: &'static [(i32, i32)] = &[(1, 0), (0, -1), (-1, 0), (0, 1), (1, 0)];

    fn dijkstra(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();

        let mut dist = vec![vec![i32::MAX; m]; n];
        dist[0][0] = grid[0][0];

        let mut visited = vec![vec![false; m]; n];
        let mut queue = BinaryHeap::new();

        queue.push(Reverse((dist[0][0], (0, 0))));
        while let Some(Reverse((d, (i, j)))) = queue.pop() {
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            for (to_i, to_j) in Solution::DIRS.iter().filter_map(|(di, dj)| {
                let to_i = (i as i32 + di) as usize;
                let to_j = (j as i32 + dj) as usize;
                let _ = grid.get(to_i)?.get(to_j)?;
                Some((to_i, to_j))
            }) {
                if dist[to_i][to_j] > d + grid[to_i][to_j] {
                    dist[to_i][to_j] = d + grid[to_i][to_j];
                    queue.push(Reverse((dist[to_i][to_j], (to_i, to_j))));
                }
            }
        }
        dist
    }

    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        Solution::dijkstra(&grid)[n - 1][m - 1]
    }
}
