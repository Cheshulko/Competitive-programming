// https://leetcode.com/problems/find-resultant-array-after-removing-anagrams

struct Solution {}

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut words_sort = words
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, w)| {
                let mut w = w.into_bytes();
                w.sort_unstable();

                (w, i)
            })
            .rev()
            .collect::<Vec<_>>();

        let mut ans = vec![];
        while let Some((w, i)) = words_sort.pop() {
            ans.push((w, i));

            while ans.len() > 1 {
                let l = ans.len();

                if ans[l - 1].0 == ans[l - 2].0 {
                    ans.pop();
                } else {
                    break;
                }
            }
        }

        ans.into_iter().map(|(_, i)| words[i].clone()).collect()
    }
}
