// https://leetcode.com/problems/trionic-array-ii

struct Solution;

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(i64::from).collect::<Vec<_>>();

        let n = nums.len();

        let incs = {
            let mut incs = vec![];
            let mut len = 1;
            for i in 1..n {
                if nums[i] > nums[i - 1] {
                    len += 1;
                } else {
                    if len > 1 {
                        incs.push((i - len, i - 1));
                    }
                    len = 1;
                }
            }
            if len > 1 {
                incs.push((n - len, n - 1));
            }

            incs
        };

        let decs = {
            let mut decs = vec![];
            let mut len = 1;
            for i in 1..n {
                if nums[i] < nums[i - 1] {
                    len += 1;
                } else {
                    if len > 1 {
                        decs.push((i - len, i - 1));
                    }
                    len = 1;
                }
            }
            if len > 1 {
                decs.push((n - len, n - 1));
            }

            decs
        };

        let mut best = i64::MIN;
        for (st_d, en_d) in decs.into_iter() {
            let i = incs.partition_point(|&(st_i, _)| st_i < st_d);
            let inc_1 = if i > 0 && incs[i - 1].1 == st_d {
                incs[i - 1]
            } else {
                continue;
            };

            let i = incs.partition_point(|&(st_i, _)| st_i < en_d);
            let inc_2 = if i != incs.len() && incs[i].0 == en_d {
                incs[i]
            } else {
                continue;
            };

            assert!(inc_1.1 - inc_1.0 >= 1);
            let mut inc_1_sum = nums[inc_1.1] + nums[inc_1.1 - 1];
            let mut inc_1_max = inc_1_sum;
            for i in (inc_1.0..=inc_1.1).rev().skip(2) {
                inc_1_sum += nums[i];
                inc_1_max = inc_1_max.max(inc_1_sum)
            }

            assert!(inc_2.1 - inc_2.0 >= 1);
            let mut inc_2_sum = nums[inc_2.0] + nums[inc_2.0 + 1];
            let mut inc_2_max = inc_2_sum;
            for i in (inc_2.0..=inc_2.1).skip(2) {
                inc_2_sum += nums[i];
                inc_2_max = inc_2_max.max(inc_2_sum)
            }

            let dec_sum = ((st_d + 1)..en_d).map(|i| nums[i]).sum::<i64>();

            best = best.max(inc_1_max + dec_sum + inc_2_max);
        }

        best
    }
}
