// https://leetcode.com/problems/find-minimum-time-to-reach-last-room-i

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        fn dijkstra(move_time: Vec<Vec<i32>>, (si, sj): (usize, usize)) -> Vec<Vec<i32>> {
            let n = move_time.len();
            let m = move_time[0].len();

            let mut dist = vec![vec![i32::MAX; m]; n];
            dist[si][sj] = 0;

            let mut visited = vec![vec![false; m]; n];
            let mut queue = BinaryHeap::new();

            queue.push(Reverse((0, si, sj)));
            while let Some(Reverse((d, i, j))) = queue.pop() {
                if visited[i][j] {
                    continue;
                }
                visited[i][j] = true;

                for (t, to_i, to_j) in
                    [(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .iter()
                        .filter_map(|(di, dj)| {
                            let to_i = (i as i32 + *di) as usize;
                            let to_j = (j as i32 + *dj) as usize;
                            let t = *move_time.get(to_i)?.get(to_j)?;

                            Some((t, to_i, to_j))
                        })
                {
                    let to_t = d.max(t) + 1;
                    if dist[to_i][to_j] > to_t {
                        dist[to_i][to_j] = to_t;
                        queue.push(Reverse((dist[to_i][to_j], to_i, to_j)));
                    }
                }
            }
            dist
        }

        let n = move_time.len();
        let m = move_time[0].len();

        dijkstra(move_time, (0, 0))[n - 1][m - 1]
    }
}
