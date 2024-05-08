// https://leetcode.com/problems/relative-ranks

struct Solution {}

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut score = score
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<Vec<_>>();

        score.sort_unstable();
        score.reverse();

        let n = score.len();
        score
            .into_iter()
            .enumerate()
            .fold(vec![String::default(); n], |mut a, (i, (_, p))| {
                a[p] = match i {
                    0 => "Gold Medal".to_string(),
                    1 => "Silver Medal".to_string(),
                    2 => "Bronze Medal".to_string(),
                    x => (x + 1).to_string(),
                };
                a
            })
    }
}
