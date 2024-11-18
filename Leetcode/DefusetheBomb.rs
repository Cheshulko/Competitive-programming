// https://leetcode.com/problems/defuse-the-bomb

struct Solution {}

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut ans = vec![0; n];
        match k {
            0 => {}
            1.. => {
                for i in 0..n {
                    let mut s = 0;
                    let mut j = (n + i + 1) % n;
                    for _ in 0..k.abs() as usize {
                        s += code[j];
                        j = (n + j + 1) % n;
                    }
                    ans[i] = s;
                }
            }
            _ => {
                for i in 0..n {
                    let mut s = 0;
                    let mut j = (n + i - 1) % n;
                    for _ in 0..k.abs() as usize {
                        s += code[j];
                        j = (n + j - 1) % n;
                    }
                    ans[i] = s;
                }
            }
        };
        ans
    }
}
