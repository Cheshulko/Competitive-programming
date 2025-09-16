// https://leetcode.com/problems/replace-non-coprime-numbers-in-array

struct Solution {}

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            use std::mem::swap;

            while a != 0 {
                if a < b {
                    swap(&mut a, &mut b);
                }
                a %= b;
            }
            b
        }

        let mut ans = vec![];
        for num in nums {
            ans.push(num);

            while ans.len() > 1 {
                let l = ans.len();
                let [a, b] = [ans[l - 1], ans[l - 2]];
                let g = gcd(a, b);
                if g > 1 {
                    ans.pop();
                    ans.pop();
                    ans.push(a / g * b);
                } else {
                    break;
                }
            }
        }

        ans
    }
}
