// https://leetcode.com/problems/find-the-town-judge

struct Solution {}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let (v, u) =
            trust
                .into_iter()
                .fold((vec![0; n + 1], vec![0; n + 1]), |(mut v, mut u), x| {
                    v[x[1] as usize] += 1;
                    u[x[0] as usize] += 1;
                    (v, u)
                });

        (1..=n)
            .filter(|i| v[*i] == n - 1 && u[*i] == 0)
            .map(|x| x as i32)
            .last()
            .unwrap_or(-1)
    }
}
