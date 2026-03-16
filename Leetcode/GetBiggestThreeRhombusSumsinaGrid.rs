// https://leetcode.com/problems/get-biggest-three-rhombus-sums-in-a-grid

struct Solution;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();

        use std::collections::BTreeSet;

        let mut ans = BTreeSet::new();
        for i in 0..n {
            for j in 0..m {
                ans.insert(grid[i][j]);

                for step in 1..n {
                    let mut sum = 0;

                    let j2 = j + 2 * step;
                    if j2 >= m {
                        continue;
                    }
                    if i < step {
                        continue;
                    }
                    if i + step >= n {
                        continue;
                    }

                    for s in 0..=step {
                        sum += grid[i - s][j + s];
                        sum += grid[i - s][j2 - s];
                        sum += grid[i + s][j + s];
                        sum += grid[i + s][j2 - s];
                    }
                    sum -= grid[i][j];
                    sum -= grid[i][j2];
                    sum -= grid[i - step][j + step];
                    sum -= grid[i + step][j + step];

                    ans.insert(sum);
                }
            }
        }

        ans.into_iter().rev().take(3).collect::<Vec<_>>()
    }
}
