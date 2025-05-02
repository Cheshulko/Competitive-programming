// https://leetcode.com/problems/push-dominoes

struct Solution {}

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let n = dominoes.len();
        let dominoes = dominoes.chars();

        let mut ans = vec!['.'; n];

        let mut v = vec![];
        let mut r = -1;
        for (j, c) in dominoes.enumerate() {
            match c {
                'R' => {
                    while let Some(_) = v.pop() {}
                    r = j as i32;
                    ans[j] = 'R';
                }
                'L' => {
                    if r != -1 {
                        let r = r as usize;
                        let m = r + (1 + j - r) / 2;
                        for k in m..=j {
                            ans[k] = 'L';
                        }
                        if r % 2 == j % 2 {
                            ans[m] = '.';
                        }
                    } else {
                        v.push(j);
                        while let Some(p) = v.pop() {
                            ans[p] = 'L';
                        }
                    }
                    r = -1;
                }
                '.' => {
                    if r != -1 {
                        ans[j] = 'R';
                    } else {
                        v.push(j);
                    }
                }
                _ => unreachable!(),
            }
        }

        ans.into_iter().collect()
    }
}
