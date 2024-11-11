// https://leetcode.com/problems/prime-subtraction-operation

struct Solution {}

impl Solution {
    fn eratosfen(n: usize) -> Vec<bool> {
        let mut pr = vec![true; n + 1];
        pr[0] = true;
        pr[1] = false;
        for i in 2..=n {
            if pr[i] {
                for j in (2 * i..=n).step_by(i) {
                    pr[j] = false;
                }
            }
        }
        pr
    }

    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let pr = Solution::eratosfen(1000);
        let pr = pr
            .into_iter()
            .enumerate()
            .filter_map(|(i, x)| x.then_some(i))
            .collect::<Vec<_>>();

        let mut prev = 0;
        for x in nums.into_iter() {
            let x = x as usize;
            if x <= prev {
                return false;
            }

            let p = pr.partition_point(|&y| y < x - prev) - 1;
            prev = x - pr[p];
        }

        true
    }
}
