// https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-ii

struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        use std::collections::BTreeSet;

        let n = nums.len();
        let k = k as usize;
        let dist = dist as usize;

        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let mut can: BTreeSet<(i64, usize)> = BTreeSet::new();
        let mut cannot: BTreeSet<(i64, usize)> = BTreeSet::new();
        let mut sum = nums[0];

        let mut j = 0;
        let mut ans = i64::MAX;
        for i in 0..n {
            while j <= i {
                let p = (nums[j], j);
                cannot.remove(&p);
                if can.contains(&p) {
                    sum -= p.0;
                    can.remove(&p);
                }

                j += 1;
            }

            while j - i <= dist && j < n {
                cannot.insert((nums[j], j));
                j += 1;
            }

            while let Some(next) = cannot.first().copied() {
                if can.len() == k - 1 {
                    if let Some(worst) = can.last().copied() {
                        if worst.0 > next.0 {
                            cannot.pop_first();
                            can.pop_last();

                            sum -= worst.0;
                            sum += next.0;

                            cannot.insert(worst);
                            can.insert(next);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    cannot.pop_first();
                    can.insert(next);
                    sum += next.0;
                }
            }

            if can.len() == k - 1 {
                ans = ans.min(sum);
            }

            let p = (nums[i], i);
            cannot.remove(&p);
            if can.contains(&p) {
                sum -= p.0;
                can.remove(&p);
            }
        }

        ans
    }
}
