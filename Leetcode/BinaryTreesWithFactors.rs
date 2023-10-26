// https://leetcode.com/problems/binary-trees-with-factors

use std::collections::{BTreeMap, BTreeSet};

struct Solution {}

impl Solution {
    const MOD: i128 = 1_000_000_000 + 7;

    fn dfs(
        cnt: &mut BTreeMap<i128, i128>,
        ans: &mut i128,
        children: &BTreeMap<i128, Vec<(i128, i128)>>,
        cur: i128,
    ) -> i128 {
        if let Some(cur_ans) = cnt.get(&cur) {
            return *cur_ans;
        }

        let mut cur_ans = 0;

        if let Some(cur_children_pair) = children.get(&cur) {
            for children_pair in cur_children_pair {
                let left = Solution::dfs(cnt, ans, children, children_pair.0);
                let right = Solution::dfs(cnt, ans, children, children_pair.1);

                let mut pair_ans = 1;
                pair_ans = (pair_ans * left) % Solution::MOD;
                pair_ans = (pair_ans * right) % Solution::MOD;
                cur_ans = (cur_ans + pair_ans) % Solution::MOD;
            }
        }

        cur_ans = (1 + cur_ans) % Solution::MOD;
        cnt.insert(cur, cur_ans);

        return cur_ans;
    }

    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let arr = arr.into_iter().map(|x| x as i128).collect::<Vec<_>>();

        let mut nums: BTreeSet<i128> = BTreeSet::<i128>::new();
        let mut children = BTreeMap::<i128, Vec<(i128, i128)>>::new();
        let mut cnt = BTreeMap::<i128, i128>::new();

        for num in arr.iter() {
            nums.insert(*num);
        }

        for i in 0..n {
            for j in i..n {
                let pr = arr[i] * arr[j];
                if nums.contains(&pr) {
                    let r = children.entry(pr).or_insert(vec![]);
                    r.push((arr[i], arr[j]));
                    if arr[i] != arr[j] {
                        r.push((arr[j], arr[i]));
                    }
                }
            }
        }

        let mut ans = 0;
        for x in arr.into_iter() {
            ans = (ans + Solution::dfs(&mut cnt, &mut ans, &children, x as i128)) % Solution::MOD;
        }

        ans as i32
    }
}
