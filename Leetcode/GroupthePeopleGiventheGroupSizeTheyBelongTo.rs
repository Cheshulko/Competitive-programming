// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to

struct Solution {}

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let n = group_sizes.len();
        let mut cnt = vec![0; n + 1];
        let mut ans_group_index: Vec<Option<usize>> = vec![None; n + 1];

        for group in &group_sizes {
            let group = *group as usize;
            cnt[group] += 1;
        }

        let mut ans: Vec<Vec<i32>> = vec![];
        for (i, group) in group_sizes.into_iter().enumerate() {
            let group = group as usize;

            if ans_group_index[group].is_none() {
                ans_group_index[group] = Some(ans.len());
                ans.push(vec![]);
            }

            let ind = ans_group_index[group].unwrap();
            ans[ind].push(i as i32);
            if ans[ind].len() == group {
                ans_group_index[group] = None;
            }
        }

        ans
    }
}
