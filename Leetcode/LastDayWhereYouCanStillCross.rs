// https://leetcode.com/problems/last-day-where-you-can-still-cross

mod cm {
    pub struct DSU {
        parents: Vec<usize>,
        ranks: Vec<usize>,
    }

    impl DSU {
        pub fn new(size: usize) -> Self {
            Self {
                parents: (0..size).collect(),
                ranks: vec![1; size],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if x != self.parents[x] {
                self.parents[x] = self.find(self.parents[x]);
            }

            self.parents[x]
        }

        pub fn same(&mut self, mut x: usize, mut y: usize) -> bool {
            x = self.find(x);
            y = self.find(y);

            x == y
        }

        pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
            x = self.find(x);
            y = self.find(y);

            if x == y {
                return false;
            }

            if self.ranks[x] < self.ranks[y] {
                std::mem::swap(&mut y, &mut x);
            }

            self.parents[y] = x;
            self.ranks[x] += self.ranks[y];

            true
        }
    }
}

struct Solution {}

impl Solution {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let col = col as usize;
        let row = row as usize;
        let n = col * row;

        let mut grid = vec![vec![0; col]; row];

        cells
            .iter()
            .for_each(|c| grid[(c[0] - 1) as usize][(c[1] - 1) as usize] = 1);

        let mut dsu = cm::DSU::new(n + 2);

        fn get_dirs(i: i32, j: i32, grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
            Solution::DIRS
                .iter()
                .filter_map(|dir| {
                    let to: (usize, usize) = ((i + dir.0) as usize, (j + dir.1) as usize);
                    grid.get(to.0)?.get(to.1).filter(|c| c == &&0).map(|_| to)
                })
                .collect::<Vec<_>>()
        }

        (0..row as i32).zip(0..col as i32).for_each(|(i, j)| {
            for dir in get_dirs(i, j, &grid).into_iter() {
                dsu.union(i as usize * col + j as usize, dir.0 * col + dir.1);
            }
        });

        let common_start = n;
        let common_end = n + 1;

        for j in 0..col {
            dsu.union(common_start, 0 * col + j);
            dsu.union(common_end, (row - 1) * col + j);
        }

        if dsu.same(common_start, common_end) {
            return cells.len() as i32;
        }

        for (ind, cell) in cells.iter().rev().enumerate() {
            let i = cell[0] - 1;
            let j = cell[1] - 1;
            grid[i as usize][j as usize] = 0;

            for dir in get_dirs(i, j, &grid) {
                dsu.union(i as usize * col + j as usize, dir.0 * col + dir.1);
            }

            if dsu.same(common_start, common_end) {
                return (cells.len() - ind - 1) as i32;
            }
        }

        0
    }
}
