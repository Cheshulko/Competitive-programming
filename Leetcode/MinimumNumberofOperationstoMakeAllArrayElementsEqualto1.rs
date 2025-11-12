// https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            if a == 0 {
                return b;
            }
            if b == 0 {
                return a;
            }

            while a != 0 {
                if a < b {
                    std::mem::swap(&mut a, &mut b);
                }
                a %= b;
            }
            b
        }

        if nums.iter().all(|&x| x == 1) {
            return 0;
        }
        if nums.iter().any(|&x| x == 1) {
            return nums.iter().filter(|&&x| x != 1).count() as i32;
        }

        let n = nums.len();

        let mut d = usize::MAX;
        for i in 0..n {
            let mut cur = nums[i];
            for j in i..n {
                cur = gcd(cur, nums[j]);
                if cur == 1 {
                    d = d.min(j - i);
                    break;
                }
            }
        }

        if d == usize::MAX {
            return -1;
        } else {
            return (d + n - 1) as i32;
        }
    }
}
