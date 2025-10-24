// https://leetcode.com/problems/next-greater-numerically-balanced-number

struct Solution {}

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        fn check(mut n: usize) -> bool {
            let mut cnt = [0; 10];
            while n > 0 {
                cnt[n % 10] += 1;
                n /= 10
            }

            cnt.into_iter()
                .enumerate()
                .all(|(i, c)| c == 0 || (c > 0 && i == c))
        }

        for i in (n as usize) + 1.. {
            if check(i) {
                return i as i32;
            }
        }

        unreachable!()
    }
}
