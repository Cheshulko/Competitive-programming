// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k

struct Solution {}

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut rem = vec![0; k as usize];

        for &x in arr.iter() {
            rem[((x % k + k) % k) as usize] += 1;
        }

        for x in arr.into_iter() {
            let r1 = (x % k + k) % k;
            if rem[r1 as usize] > 0 {
                rem[r1 as usize] -= 1;

                let r2 = (k - r1) % k;
                if rem[r2 as usize] > 0 {
                    rem[r2 as usize] -= 1;
                    continue;
                }

                return false;
            } else {
                continue;
            }
        }

        return true;
    }
}
