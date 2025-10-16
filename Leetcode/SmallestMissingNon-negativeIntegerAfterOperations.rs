// https://leetcode.com/problems/smallest-missing-non-negative-integer-after-operations

struct Solution {}

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let ma = value as usize;

        let mut cnt = nums.into_iter().fold(vec![0; ma], |mut v, x| {
            let value = ((x % value) + value) % value;
            v[value as usize] += 1;
            v
        });

        for ans in 0.. {
            if cnt[ans % ma] > 0 {
                cnt[ans % ma] -= 1;
            } else {
                return ans as i32;
            }
        }

        unreachable!()
    }
}
