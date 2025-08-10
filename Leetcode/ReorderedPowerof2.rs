// https://leetcode.com/problems/reordered-power-of-2

struct Solution {}

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut n = n as i64;

        let mut narr = vec![];
        while n > 0 {
            narr.push(n % 10);
            n /= 10;
        }
        narr.sort_unstable();

        let mut i = 1;
        loop {
            let mut j = i;
            let mut a = vec![];
            while j > 0 {
                a.push(j % 10);
                j /= 10;
            }
            if a.len() > narr.len() {
                break;
            }
            a.sort_unstable();
            if a == narr {
                return true;
            }

            i <<= 1;
        }

        false
    }
}
