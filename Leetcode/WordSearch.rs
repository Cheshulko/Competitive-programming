// https://leetcode.com/problems/word-search

struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(
            board: &Vec<Vec<char>>,
            word: &Vec<char>,
            (i, j): (usize, usize),
            pos: usize,
            used: &mut Vec<Vec<bool>>,
        ) -> bool {
            if pos == word.len() {
                return true;
            }

            used[i][j] = true;
            for (to_i, to_j) in
                [(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .into_iter()
                    .filter_map(|(di, dj)| {
                        let to_i = (i as i32 + di) as usize;
                        let to_j = (j as i32 + dj) as usize;
                        let to_c = board.get(to_i)?.get(to_j)?;
                        (to_c == &word[pos]).then_some((to_i, to_j))
                    })
            {
                if !used[to_i][to_j] && dfs(board, word, (to_i, to_j), pos + 1, used) {
                    return true;
                }
            }

            used[i][j] = false;

            false
        }

        let word = word.chars().collect::<Vec<_>>();

        let mut used = vec![vec![false; board[0].len()]; board.len()];

        for (i, r) in board.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if c == &word[0] {
                    if dfs(&board, &word, (i, j), 1, &mut used) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
