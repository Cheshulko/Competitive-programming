// https://leetcode.com/problems/build-a-matrix-with-conditions

mod cm {
    fn dfs(cur: usize, edges: &Vec<Vec<usize>>, times: &Vec<usize>, used: &mut Vec<bool>) -> bool {
        used[cur] = true;
        let mut ok = true;

        for to in &edges[cur] {
            if times[*to] < times[cur] {
                return false;
            }
            if !used[*to] {
                ok &= dfs(*to, edges, times, used);
            }
        }

        return ok;
    }

    fn dfs_topological(
        cur: usize,
        edges: &Vec<Vec<usize>>,
        used: &mut Vec<bool>,
        ans: &mut Vec<usize>,
    ) {
        used[cur] = true;
        for to in &edges[cur] {
            if !used[*to] {
                dfs_topological(*to, edges, used, ans);
            }
        }
        ans.push(cur);
    }

    pub fn topological_sort(n: usize, edges: &Vec<Vec<usize>>) -> (Vec<usize>, bool) {
        let mut ans: Vec<usize> = vec![];
        let mut used = vec![false; n];

        for to in 0..n {
            if !used[to] {
                dfs_topological(to, edges, &mut used, &mut ans);
            }
        }

        ans.reverse();
        let mut times = vec![0; n + 1];
        for i in 0..ans.len() {
            times[ans[i]] = i;
        }

        let mut ok = true;
        let mut used = vec![false; n];
        for to in 0..n {
            let to = ans[to];
            if !used[to] {
                ok &= dfs(to, edges, &times, &mut used);
            }
        }

        (ans, ok)
    }
}

struct Solution {}

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let k = k as usize;

        let mut pos = vec![(0, 0); k + 1];
        let mut ans = vec![vec![0; k]; k];
        let mut ok = true;

        {
            let mut adj = vec![vec![]; k + 1];
            for con in row_conditions.into_iter() {
                adj[con[1] as usize].push(con[0] as usize);
            }
            let (sort_rows, ok_rows) = cm::topological_sort(k + 1, &adj);
            ok &= ok_rows;

            let mut p = k - 1;
            for i in 0..sort_rows.len() - 1 {
                let x = sort_rows[i];
                pos[x].1 = p;
                p -= 1;
            }
        }
        {
            let mut adj = vec![vec![]; k + 1];
            for con in col_conditions.into_iter() {
                adj[con[1] as usize].push(con[0] as usize);
            }
            let (sort_cols, ok_cols) = cm::topological_sort(k + 1, &adj);
            ok &= ok_cols;

            let mut p = k - 1;
            for i in 0..sort_cols.len() - 1 {
                let x = sort_cols[i];
                pos[x].0 = p;
                p -= 1;
            }
        }

        if ok {
            for (i, (x, y)) in pos.into_iter().enumerate() {
                ans[y][x] = i as i32;
            }
            ans
        } else {
            vec![]
        }
    }
}
