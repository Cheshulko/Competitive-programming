// https://leetcode.com/problems/pacific-atlantic-water-flow

struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const DIRS: &[(i32, i32)] = &[(1, 0), (0, -1), (-1, 0), (0, 1), (1, 0)];

        let h = heights.len();
        let w = heights[0].len();

        let mut pacific = vec![vec![false; w]; h];
        for j in 0..w {
            pacific[0][j] = true;
        }
        for i in 1..h {
            pacific[i][0] = true;
        }

        let mut atlantic = vec![vec![false; w]; h];
        for j in 0..w {
            atlantic[h - 1][j] = true;
        }
        for i in 0..h - 1 {
            atlantic[i][w - 1] = true;
        }

        for ocean in [&mut pacific, &mut atlantic] {
            let mut q = vec![];
            for i in 0..h {
                for j in 0..w {
                    if (*ocean)[i][j] {
                        q.push((i, j));
                    }
                }
            }

            while let Some((i, j)) = q.pop() {
                (*ocean)[i][j] = true;
                let h = heights[i][j];

                for (to_i, to_j) in DIRS.iter().filter_map(|(di, dj)| {
                    let to_i = (i as i32 + di) as usize;
                    let to_j = (j as i32 + dj) as usize;

                    let to_h = *heights.get(to_i)?.get(to_j)?;
                    (to_h >= h).then_some((to_i, to_j))
                }) {
                    if !(*ocean)[to_i][to_j] {
                        (*ocean)[to_i][to_j] = true;
                        q.push((to_i, to_j));
                    }
                }
            }
        }

        pacific
            .into_iter()
            .zip(atlantic.into_iter())
            .enumerate()
            .flat_map(|(i, (r1, r2))| {
                r1.into_iter()
                    .zip(r2.into_iter())
                    .enumerate()
                    .filter_map(|(j, (c1, c2))| {
                        (c1 == c2 && c1 == true).then_some(vec![i as i32, j as i32])
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}
