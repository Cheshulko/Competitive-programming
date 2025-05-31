// https://leetcode.com/problems/snakes-and-ladders

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        fn step((i, j): (usize, usize), board: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
            let i = i as i32;
            let j = j as i32;
            let n = board.len() as i32;

            let right = ((i ^ (n - 1)) & 1) == 0;

            let to = if j == 0 {
                if right {
                    (i, j + 1)
                } else {
                    (i - 1, j)
                }
            } else if j == n - 1 {
                if right {
                    (i - 1, j)
                } else {
                    (i, j - 1)
                }
            } else {
                if right {
                    (i, j + 1)
                } else {
                    (i, j - 1)
                }
            };

            if to.0 < 0 || to.1 < 0 {
                return None;
            }

            let to = (to.0 as usize, to.1 as usize);
            return Some(to);
        }

        fn jump(cur @ (i, j): (usize, usize), board: &Vec<Vec<i32>>) -> (usize, usize) {
            if board[i][j] == -1 {
                return cur;
            }

            let b = board[i][j] as usize;
            let n = board[0].len();

            let di = (b - 1) / n;
            let i_ = n - 1 - di;
            let dj = (b - 1) % n;

            let j_ = if di % 2 == 0 { dj } else { n - 1 - dj };

            return (i_, j_);
        }

        let n = board[0].len();

        let mut bh = BinaryHeap::new();
        let mut best = vec![vec![i32::MAX; n]; n];
        best[n - 1][0] = 0;
        bh.push(Reverse((0, (n - 1, 0))));

        while let Some(Reverse((d, cur @ (i, j)))) = bh.pop() {
            if best[i][j] < d {
                continue;
            }

            let mut to = cur;
            for _ in 1..=6 {
                if let Some(to_) = step(to, &board) {
                    to = to_;

                    let to_jumped = jump(to, &board);

                    if best[to_jumped.0][to_jumped.1] > d + 1 {
                        best[to_jumped.0][to_jumped.1] = d + 1;
                        bh.push(Reverse((d + 1, to_jumped)));
                    }
                } else {
                    break;
                }
            }
        }

        let ans = best[0][(n - 1) * (n % 2 == 1) as usize];
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
