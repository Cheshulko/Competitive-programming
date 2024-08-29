// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column

struct Solution {}

impl Solution {
    fn dfs(
        cur: usize,
        rows: &Vec<Vec<usize>>,
        cols: &Vec<Vec<usize>>,
        stones: &Vec<Vec<i32>>,
        used: &mut Vec<bool>,
    ) -> usize {
        used[cur] = true;
        let (x, y) = (stones[cur][0] as usize, stones[cur][1] as usize);

        let mut cnt = 0;
        for &i in rows[x].iter() {
            if i != cur && !used[i] {
                cnt += Solution::dfs(i, rows, cols, stones, used);
            }
        }
        for &i in cols[y].iter() {
            if i != cur && !used[i] {
                cnt += Solution::dfs(i, rows, cols, stones, used);
            }
        }

        1 + cnt
    }

    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut rows = vec![vec![]; 10_000 + 1];
        let mut cols = vec![vec![]; 10_000 + 1];
        for (i, stone) in stones.iter().enumerate() {
            rows[stone[0] as usize].push(i);
            cols[stone[1] as usize].push(i);
        }

        let mut used = vec![false; stones.len()];

        let mut ans = 0;
        for i in 0..stones.len() {
            if !used[i] {
                ans += Solution::dfs(i, &rows, &cols, &stones, &mut used) - 1;
            }
        }

        ans as i32
    }
}
