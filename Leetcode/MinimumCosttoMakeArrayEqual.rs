// https://leetcode.com/problems/minimum-cost-to-make-array-equal

struct Solution {}

impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let cost = cost.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let mut nums = nums
            .into_iter()
            .enumerate()
            .map(|(ind, v)| (v, ind))
            .collect::<Vec<_>>();

        nums.sort_by(|a, b| a.0.cmp(&b.0));

        let mut pref: Vec<i64> = vec![0; 1];
        nums.iter().enumerate().for_each(|(ind_out, (_, ind))| {
            pref.push(pref[ind_out] + cost[*ind]);
        });

        let mut suf: Vec<i64> = vec![0; 1];
        nums.iter()
            .rev()
            .enumerate()
            .for_each(|(ind_out, (_, ind))| {
                suf.push(suf[ind_out] + cost[*ind]);
            });

        let mut arr_suf: Vec<i64> = vec![0; 1];
        for i in (1..nums.len()).rev() {
            arr_suf.push(
                arr_suf.last().unwrap() + (nums[i].0 - nums[i - 1].0) * suf[suf.len() - 1 - i],
            );
        }

        let mut arr_pref: Vec<i64> = vec![0; 1];
        for i in 0..nums.len() - 1 {
            arr_pref.push(arr_pref.last().unwrap() + (nums[i + 1].0 - nums[i].0) * pref[i + 1]);
        }

        let mut ans = i64::MAX;
        for i in 0..nums.len() {
            ans = ans.min(arr_suf[nums.len() - 1 - i] + arr_pref[i]);
        }

        ans
    }
}
