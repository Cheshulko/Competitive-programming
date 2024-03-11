// https://leetcode.com/problems/custom-sort-string

struct Solution {}

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let order =
            order
                .as_bytes()
                .iter()
                .enumerate()
                .fold(vec![usize::MAX; 150], |mut v, (i, c)| {
                    v[*c as usize] = i;
                    v
                });

        let mut s = s.chars().collect::<Vec<_>>();
        s.sort_by(|a, b| order[*a as usize].cmp(&order[*b as usize]));
        s.into_iter().collect::<String>()
    }
}
