// https://leetcode.com/contest/biweekly-contest-104/problems/maximum-or/

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let mx = nums
            .iter()
            .map(|x| (*x as f64).log2() as i64)
            .max()
            .unwrap();

        let mx = 2_u64.pow(mx as u32) as i64;

        let mut pr = vec![0; nums.len()];
        let mut su = vec![0; nums.len()];

        for i in 0..nums.len() {
            if i > 0 {
                pr[i] = pr[i - 1] | nums[i];
            } else {
                pr[i] = nums[i];
            }
        }

        for i in (0..nums.len()).rev() {
            if i + 1 < nums.len() {
                su[i] = su[i + 1] | nums[i];
            } else {
                su[i] = nums[nums.len() - 1];
            }
        }

        let x = nums
            .iter()
            .enumerate()
            .filter(|(ind, y)| (**y as i64) & mx != 0)
            .collect::<Vec<(usize, &i32)>>();

        let mut ans: i64 = 0;
        let po: i64 = 2_u64.pow(k as u32) as i64;

        x.iter().for_each(|(ind, x)| {
            let mut cur: i64 = 0;
            if *ind > 0 {
                cur |= (pr[*ind - 1] as i64);
            }

            if *ind + 1 < nums.len() {
                cur |= (su[*ind + 1] as i64);
            }

            cur |= (nums[*ind] as i64 * po);

            ans = ans.max(cur);
        });

        ans
    }
}
