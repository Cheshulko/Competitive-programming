// https://leetcode.com/problems/find-the-winner-of-the-circular-game

struct Solution {}

impl Solution {
    pub fn find_the_winner(n: i32, mut k: i32) -> i32 {
        let mut k = k as usize;
        let n = n as usize;

        let mut v = (1..=n).into_iter().collect::<Vec<_>>();

        let mut cur = 0;
        let mut cnt = n;

        while cnt != 1 {
            let mut i = cur;
            let mut need = k;

            while need != 0 {
                if v[i] != 0 {
                    need -= 1;
                }
                if need != 0 {
                    i += 1;
                    i %= n;
                }
            }

            v[i] = 0;
            cur = i;
            cnt -= 1;
        }

        v.into_iter().find(|x| x != &0).unwrap() as i32
    }
}
