use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;

struct Cin {
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self { tokens }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

const DIRS: &[(i32, i32)] = &[(1, 0), (0, -1), (-1, 0), (0, 1), (1, 0)];

fn dfs(
    (ci, cj): (usize, usize),
    grid: &Vec<Vec<u8>>,
    used: &mut Vec<Vec<bool>>,
) -> (usize, (usize, usize), (usize, usize)) {
    used[ci][cj] = true;

    let mut left = cj;
    let mut right = cj;
    let mut up = ci;
    let mut down = ci;

    let mut cnt = 0;
    for (to_i, to_j) in DIRS.iter().filter_map(|(di, dj)| {
        let to_i = (ci as i32 + di) as usize;
        let to_j = (cj as i32 + dj) as usize;
        (grid.get(to_i)?.get(to_j)? == &b'#').then_some((to_i, to_j))
    }) {
        if used[to_i][to_j] {
            continue;
        }

        let (c, (l, r), (u, d)) = dfs((to_i, to_j), grid, used);

        left = left.min(l);
        right = right.max(r);
        up = up.min(u);
        down = down.max(d);
        cnt += c;
    }

    (1 + cnt, (left, right), (up, down))
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();

    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut grid = vec![];
        let mut free_in_rows = vec![0; n];
        let mut free_in_cols = vec![0; m];
        for i in 0..n {
            let x = cin.next::<String>().into_bytes();
            for j in 0..m {
                free_in_rows[i] += (x[j] == b'.') as usize;
                free_in_cols[j] += (x[j] == b'.') as usize;
            }

            grid.push(x);
        }

        let mut used = vec![vec![false; m]; n];
        let mut rows = vec![0; n];
        let mut cols = vec![0; m];

        for i in 0..n {
            for j in 0..m {
                if !used[i][j] && grid[i][j] == b'#' {
                    let (c, (l, r), (u, d)) = dfs((i, j), &grid, &mut used);

                    for k in l..=r {
                        cols[k] += c;
                    }
                    if l > 0 {
                        cols[l - 1] += c;
                    }
                    if r + 1 < m {
                        cols[r + 1] += c;
                    }

                    for k in u..=d {
                        rows[k] += c;
                    }
                    if u > 0 {
                        rows[u - 1] += c;
                    }
                    if d + 1 < n {
                        rows[d + 1] += c;
                    }
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(rows[i] + free_in_rows[i]);
        }
        for j in 0..m {
            ans = ans.max(cols[j] + free_in_cols[j]);
        }

        println!("{ans}");
    }
}
