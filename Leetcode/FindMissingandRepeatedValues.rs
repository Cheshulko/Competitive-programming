// https://leetcode.com/problems/find-missing-and-repeated-values

struct Solution {}

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut seen = vec![0; n * n + 1];
        seen[0] = 1;
        for r in grid.into_iter() {
            for x in r.into_iter() {
                seen[x as usize] += 1;
            }
        }

        let x = seen
            .iter()
            .enumerate()
            .filter_map(|(i, x)| (*x == 0).then_some(i))
            .next()
            .unwrap();
        let y = seen
            .iter()
            .enumerate()
            .filter_map(|(i, x)| (*x == 2).then_some(i))
            .next()
            .unwrap();

        vec![y as i32, x as i32]
    }
}
