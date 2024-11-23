struct Solution {}

impl Solution {
    fn rotate_clockwise(mtx: &mut Vec<Vec<char>>) {
        let n = mtx.len();
        let m = mtx[0].len();

        for i in 0..n.min(m) {
            for j in i..n.min(m) {
                let temp = mtx[i][j];
                mtx[i][j] = mtx[j][i];
                mtx[j][i] = temp;
            }
        }
        if n > m {
            for i in 0..m {
                for j in n.min(m)..n {
                    let x = mtx[j][i];
                    mtx[i].push(x);
                }
            }
            for _ in 0..m.abs_diff(n) {
                mtx.pop();
            }
        } else {
            mtx.resize(m, vec![]);
            for i in n.min(m)..m {
                for j in 0..n {
                    let x = mtx[j][i];
                    mtx[i].push(x);
                }
            }
            for i in 0..n.min(m) {
                for _ in 0..m.abs_diff(n) {
                    mtx[i].pop();
                }
            }
        }
        for i in 0..m {
            mtx[i].reverse();
        }
    }

    pub fn rotate_the_box(mut b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        Solution::rotate_clockwise(&mut b);

        let n = b.len();
        let m = b[0].len();

        fn find(b: &Vec<Vec<char>>, j: usize, cur: &mut usize) -> bool {
            while b[*cur][j] != '.' {
                if *cur == 0 {
                    return false;
                }
                *cur -= 1;
            }
            return true;
        }

        'out: for j in 0..m {
            let mut cur = n - 1;
            if !find(&b, j, &mut cur) {
                continue 'out;
            }

            for i in (0..cur).rev() {
                if i >= cur {
                    continue;
                }
                if b[i][j] == '#' {
                    b[cur][j] = '#';
                    b[i][j] = '.';
                    cur -= 1;
                } else if b[i][j] == '*' {
                    cur = i;
                    if !find(&b, j, &mut cur) {
                        continue 'out;
                    }
                }
            }
        }

        b
    }
}
