// https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid

struct Solution;

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        fn get(row: &[i32]) -> usize {
            let mut cnt = 0;
            for &x in row.iter().rev() {
                if x == 0 {
                    cnt += 1;
                } else {
                    break;
                }
            }

            cnt
        }

        let mut grid = grid
            .iter()
            .map(|row| get(row.as_slice()))
            .collect::<Vec<_>>();

        let n = grid.len();
        let mut ans = 0;
        'out: for i in 0..n {
            let need = n - 1 - i;
            for j in i..n {
                if grid[j] >= need {
                    for k in ((i + 1)..=j).rev() {
                        grid[k] = grid[k - 1];
                    }
                    ans += j - i;

                    continue 'out;
                }
            }

            return -1;
        }

        ans as i32
    }
}
