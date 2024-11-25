// https://leetcode.com/problems/sliding-puzzle

use std::collections::{HashSet, VecDeque};

struct Solution {}

impl Solution {
    const SIZE: usize = 6;

    const DIRS: [&'static [usize]; Solution::SIZE] =
        [&[1, 3], &[0, 2, 4], &[1, 5], &[0, 4], &[1, 3, 5], &[2, 4]];

    fn get_state(pos: [usize; Solution::SIZE]) -> usize {
        let mut pow = 1;
        let mut res = 0;
        for &p in pos.iter() {
            res += p * pow;
            pow *= Solution::SIZE;
        }
        res
    }

    fn get_pos(mut state: usize) -> [usize; Solution::SIZE] {
        let mut res = [0; Solution::SIZE];
        for i in 0..Solution::SIZE {
            res[i] = state % Solution::SIZE;
            state /= Solution::SIZE;
        }
        res
    }

    fn solve(state: usize) -> i32 {
        let mut seen = HashSet::new();

        let mut q = VecDeque::new();
        q.push_back((state, 0));
        seen.insert(state);

        while let Some((state, d)) = q.pop_front() {
            // 0 * 6^0 + 5 * 6^1 + 4 * 6^2 + 3 * 6^3 + 2 * 6^4 + 1 * 6^5
            if state == 11190 {
                return d;
            }

            let pos = Solution::get_pos(state);
            for i in 0..Solution::SIZE {
                if pos[i] == 0 {
                    for &dir in Solution::DIRS[i].iter() {
                        let mut next_pos = pos.clone();
                        next_pos.swap(dir, i);
                        let state = Solution::get_state(next_pos);

                        if !seen.contains(&state) {
                            seen.insert(state);
                            q.push_back((state, d + 1));
                        }
                    }

                    break;
                }
            }
        }

        return -1;
    }

    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut pos = [0; Solution::SIZE];

        for j in 0..3 {
            pos[j] = board[1][2 - j] as usize;
        }
        for j in 0..3 {
            pos[3 + j] = board[0][2 - j] as usize;
        }

        Solution::solve(Solution::get_state(pos))
    }
}
