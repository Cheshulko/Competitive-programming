// https://leetcode.com/problems/minimum-pair-removal-to-sort-array-ii

struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        use std::collections::BTreeSet;

        let mut pairs = BTreeSet::new();
        let mut wrong = 0;
        for i in 0..nums.len() - 1 {
            wrong += (nums[i] > nums[i + 1]) as usize;
            pairs.insert(((nums[i] + nums[i + 1]) as i64, i));
        }

        let mut nums = nums
            .into_iter()
            .map(|x| x as i64)
            .enumerate()
            .collect::<BTreeSet<_>>();

        let mut ans = 0;
        while wrong > 0 {
            ans += 1;

            let (sum, i) = pairs.pop_first().unwrap();

            // prev cur next next_2

            let prev = nums.range(..(i, i64::MIN)).next_back().copied();
            let cur = nums.range(..(i, i64::MAX)).next_back().copied().unwrap();

            let mut it = nums.range((i, i64::MAX)..);
            let next = it.next().copied().unwrap();
            let next_2 = it.next().copied();

            if let Some(prev) = prev {
                if prev.1 > cur.1 {
                    wrong -= 1;
                }
            }
            if cur.1 > next.1 {
                wrong -= 1;
            }
            if let Some(next_2) = next_2 {
                if next.1 > next_2.1 {
                    wrong -= 1;
                }
            }

            nums.remove(&cur);
            nums.remove(&next);
            assert!(sum == cur.1 + next.1);
            nums.insert((i, sum));

            if let Some(prev) = prev {
                pairs.remove(&(prev.1 + cur.1, prev.0));
                pairs.insert((prev.1 + sum, prev.0));

                if prev.1 > sum {
                    wrong += 1;
                }
            }

            if let Some(next_2) = next_2 {
                pairs.remove(&(next.1 + next_2.1, next.0));
                pairs.insert((sum + next_2.1, cur.0));

                if sum > next_2.1 {
                    wrong += 1;
                }
            }
        }

        ans
    }
}
