// https://leetcode.com/problems/construct-smallest-number-from-di-string

struct Solution {}

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let pattern = pattern
            .chars()
            .map(|c| (c != 'I') as usize)
            .collect::<Vec<_>>();

        let mut adj = vec![[vec![], vec![]]; 10];
        for i in 0..10 {
            for j in (i + 1)..10 {
                adj[i][0].push(j);
            }
            for j in 0..i {
                adj[i][1].push(j);
            }
        }

        fn dfs(
            pos: usize,
            cur: usize,
            pattern: &Vec<usize>,
            adj: &Vec<[Vec<usize>; 2]>,
            used: &mut Vec<bool>,
            path: &mut Vec<usize>,
        ) -> bool {
            if pos == pattern.len() {
                return true;
            }

            for &to in adj[cur][pattern[pos]].iter() {
                if !used[to] {
                    used[to] = true;
                    path.push(to);
                    if dfs(pos + 1, to, pattern, adj, used, path) {
                        return true;
                    }
                    path.pop();
                    used[to] = false;
                }
            }

            false
        }

        for cur in 0..10 {
            let mut used = vec![false; 10];
            let mut path = vec![cur];
            used[cur] = true;
            if dfs(0, cur, &pattern, &adj, &mut used, &mut path) {
                return path.into_iter().map(|x| (b'1' + x as u8) as char).collect();
            }
        }

        unreachable!()
    }
}
