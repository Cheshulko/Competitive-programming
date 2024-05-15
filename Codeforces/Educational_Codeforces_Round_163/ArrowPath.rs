use std::cmp::*;
use std::collections::*;
use std::io::stdin;

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

fn main() {
    let mut cin = Cin::new();
    let t = cin.next::<usize>();

    fn dfs((i, j): (usize, usize), grid: &Vec<Vec<u8>>, used: &mut Vec<Vec<bool>>) -> bool {
        used[i][j] = true;

        if i == 1 && j == grid[0].len() - 1 {
            return true;
        }

        let i_ = i as i32;
        let j_ = j as i32;

        let mut can = false;
        for (to_i, to_j) in [(0, 1), (-1, 0), (0, -1), (1, 0)]
            .into_iter()
            .filter_map(|(di, dj)| {
                let to_i = (i_ + di) as usize;
                let mut to_j = (j_ + dj) as usize;
                let w = *grid.get(to_i)?.get(to_j)?;

                if w == b'>' {
                    to_j += 1;
                } else {
                    to_j -= 1;
                }

                Some((to_i, to_j))
            })
        {
            if !used[to_i][to_j] {
                can |= dfs((to_i, to_j), grid, used);
            }
        }

        can
    }

    for _ in 0..t {
        let n = cin.next::<usize>();

        let mut grid = vec![];
        for _ in 0..2 {
            let x = cin.next::<String>().into_bytes();
            grid.push(x);
        }

        let mut used = vec![vec![false; n]; 2];
        let can = dfs((0, 0), &grid, &mut used);

        if can {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
