// https://leetcode.com/problems/walking-robot-simulation

use std::collections::HashSet;

struct Solution {}

impl Solution {
    const DIRS: &'static [(i32, i32)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut cur = (0, 0);

        let obstacles = obstacles
            .into_iter()
            .map(|v| (v[0], v[1]))
            .collect::<HashSet<_>>();

        let mut d = 0;
        let mut ans = 0;
        for command in commands.into_iter() {
            match command {
                -1 => d = (d + 1) % Solution::DIRS.len(),
                -2 => d = (Solution::DIRS.len() + d - 1) % Solution::DIRS.len(),
                _ => {
                    for _ in 0..command as usize {
                        let to = (cur.0 + Solution::DIRS[d].0, cur.1 + Solution::DIRS[d].1);
                        if !obstacles.contains(&to) {
                            cur = to;
                            ans = ans.max(cur.0 * cur.0 + cur.1 * cur.1);
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        ans
    }
}
