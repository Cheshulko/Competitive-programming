// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-ii

struct Solution {}

impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        fn cover_all(
            grid: &Vec<Vec<i32>>,
            from: (usize, usize),
            to: (usize, usize),
        ) -> Option<usize> {
            let (mut i0, mut i1) = (usize::MAX, usize::MIN);
            let (mut j0, mut j1) = (usize::MAX, usize::MIN);

            for i in from.0..to.0 {
                for j in from.1..to.1 {
                    if grid[i][j] == 1 {
                        i0 = i0.min(i);
                        j0 = j0.min(j);
                        i1 = i1.max(i);
                        j1 = j1.max(j);
                    }
                }
            }

            if i0 == usize::MAX {
                return None;
            }

            Some((i1 - i0 + 1) * (j1 - j0 + 1))
        }

        let n = grid.len();
        let m = grid[0].len();

        let mut ans = usize::MAX;

        /*
        |--+--|
        |P1|P2|
        |--+--|
        |P3|P4|
        |--+--|
        */
        for i in 0..n {
            for j in 0..m {
                if let (Some(p12), Some(p3), Some(p4)) = (
                    cover_all(&grid, (0, 0), (i, m)),
                    cover_all(&grid, (i, 0), (n, j)),
                    cover_all(&grid, (i, j), (n, m)),
                ) {
                    ans = ans.min(p12 + p3 + p4);
                }
                if let (Some(p13), Some(p2), Some(p4)) = (
                    cover_all(&grid, (0, 0), (n, j)),
                    cover_all(&grid, (0, j), (i, m)),
                    cover_all(&grid, (i, j), (n, m)),
                ) {
                    ans = ans.min(p13 + p2 + p4);
                }
                if let (Some(p1), Some(p2), Some(p34)) = (
                    cover_all(&grid, (0, 0), (i, j)),
                    cover_all(&grid, (0, j), (i, m)),
                    cover_all(&grid, (i, 0), (n, m)),
                ) {
                    ans = ans.min(p1 + p2 + p34);
                }
                if let (Some(p1), Some(p24), Some(p3)) = (
                    cover_all(&grid, (0, 0), (i, j)),
                    cover_all(&grid, (0, j), (n, m)),
                    cover_all(&grid, (i, 0), (n, j)),
                ) {
                    ans = ans.min(p1 + p24 + p3);
                }
            }
        }

        /*
        |--------|
        |P1|P2|P3|
        |--------|
        */
        for j0 in 0..m {
            for j1 in j0..m {
                if let (Some(p1), Some(p2), Some(p3)) = (
                    cover_all(&grid, (0, 0), (n, j0)),
                    cover_all(&grid, (0, j0), (n, j1)),
                    cover_all(&grid, (0, j1), (n, m)),
                ) {
                    ans = ans.min(p1 + p2 + p3);
                }
            }
        }

        /*
        |P1|
        |--|
        |P2|
        |--|
        |P3|
        */
        for i0 in 0..n {
            for i1 in i0..n {
                if let (Some(p1), Some(p2), Some(p3)) = (
                    cover_all(&grid, (0, 0), (i0, m)),
                    cover_all(&grid, (i0, 0), (i1, m)),
                    cover_all(&grid, (i1, 0), (n, m)),
                ) {
                    ans = ans.min(p1 + p2 + p3);
                }
            }
        }

        ans as i32
    }
}
