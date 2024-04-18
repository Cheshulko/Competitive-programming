// https://leetcode.com/problems/island-perimeter

struct Solution {}

impl Solution {
    const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let i = i as i32;
                let j = j as i32;
                if cell == &1 {
                    ans += Solution::DIRS.len()
                        - Solution::DIRS
                            .iter()
                            .filter_map(|(di, dj)| {
                                (grid.get((i + di) as usize)?.get((j + dj) as usize)? == &1)
                                    .then_some(true)
                            })
                            .count();
                }
            }
        }

        ans as i32
    }
}
