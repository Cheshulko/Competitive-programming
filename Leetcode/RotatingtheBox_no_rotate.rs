struct Solution {}

impl Solution {
    pub fn rotate_the_box(mut b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let n = b.len();
        let m = b[0].len();

        let mut ans = vec![vec![]; m];

        fn find(b: &Vec<Vec<char>>, i: usize, cur: &mut usize, ans: &mut Vec<Vec<char>>) -> bool {
            while b[i][*cur] != '.' {
                ans[*cur].push(b[i][*cur]);
                if *cur == 0 {
                    return false;
                }
                *cur -= 1;
            }
            return true;
        }

        'out: for i in (0..n).rev() {
            let mut cur = m - 1;
            if !find(&b, i, &mut cur, &mut ans) {
                continue 'out;
            }

            for j in (0..cur).rev() {
                if j >= cur {
                    continue;
                }
                if b[i][j] == '#' {
                    ans[cur].push('#');
                    b[i][j] = '.';
                    cur -= 1;
                } else if b[i][j] == '*' {
                    while cur != j {
                        ans[cur].push(b[i][cur]);
                        cur -= 1;
                    }
                    if !find(&b, i, &mut cur, &mut ans) {
                        continue 'out;
                    }
                }
            }
            loop {
                ans[cur].push(b[i][cur]);
                if cur == 0 {
                    break;
                }
                cur -= 1;
            }
        }

        ans
    }
}
