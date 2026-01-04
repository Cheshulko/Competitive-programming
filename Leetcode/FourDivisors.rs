// https://leetcode.com/problems/four-divisors

struct Solution {}

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let ma = nums.iter().max().copied().unwrap() as usize;

        let mut cnt = vec![0; ma + 1];
        let mut sum = vec![0; ma + 1];
        for i in 1..=ma {
            for j in (i..=ma).step_by(i) {
                cnt[j] += 1;
                sum[j] += i;
            }
        }

        nums.into_iter()
            .filter_map(|num| (cnt[num as usize] == 4).then_some(sum[num as usize] as i32))
            .sum()
    }
}
