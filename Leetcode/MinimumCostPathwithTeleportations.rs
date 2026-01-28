// https://leetcode.com/problems/minimum-cost-path-with-teleportations

struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::{BTreeMap, BinaryHeap};

        fn dijkstra(
            grid: Vec<Vec<i32>>,
            groups: BTreeMap<i32, Vec<(usize, usize)>>,
            k: usize,
        ) -> i32 {
            use std::ops::Bound::{Excluded, Included};

            let n = grid.len();
            let m = grid[0].len();

            let mut dist = vec![vec![vec![i32::MAX; k + 1]; m]; n];
            dist[0][0][k] = 0;

            let mut visited = vec![vec![vec![false; k + 1]; m]; n];
            let mut queue = BinaryHeap::new();

            let mut max_checked = vec![i32::MIN; k + 1];

            queue.push(Reverse((0, k, (0, 0))));
            while let Some(Reverse((d, k, (i, j)))) = queue.pop() {
                if visited[i][j][k] {
                    continue;
                }
                let any_best = dist[i][j]
                    .iter()
                    .skip(k)
                    .min()
                    .map(|&v| v < d)
                    .unwrap_or(false);
                if any_best {
                    continue;
                }

                visited[i][j][k] = true;
                if i + 1 < n {
                    let to_d = &mut dist[i + 1][j][k];
                    if *to_d > d + grid[i + 1][j] {
                        *to_d = d + grid[i + 1][j];
                        queue.push(Reverse((*to_d, k, (i + 1, j))));
                    }
                }
                if j + 1 < m {
                    let to_d = &mut dist[i][j + 1][k];
                    if *to_d > d + grid[i][j + 1] {
                        *to_d = d + grid[i][j + 1];
                        queue.push(Reverse((*to_d, k, (i, j + 1))));
                    }
                }

                if k > 0 {
                    let current_val = grid[i][j];
                    let prev_limit = max_checked[k - 1];

                    if current_val > prev_limit {
                        for (_, coords) in
                            groups.range((Excluded(prev_limit), Included(current_val)))
                        {
                            for &(ni, nj) in coords {
                                if d < dist[ni][nj][k - 1] {
                                    dist[ni][nj][k - 1] = d;
                                    queue.push(Reverse((d, k - 1, (ni, nj))));
                                }
                            }
                        }

                        max_checked[k - 1] = current_val;
                    }
                }
            }

            dist[n - 1][m - 1].iter().min().copied().unwrap()
        }

        let k = k as usize;
        let n = grid.len();
        let m = grid[0].len();

        let mut groups = BTreeMap::new();
        for i in 0..n {
            for j in 0..m {
                groups
                    .entry(grid[i][j])
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }

        dijkstra(grid, groups, k)
    }
}
