// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem

struct Solution {}

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();

        let mut ans = vec![];
        let mut last = "!";

        for f in folder {
            if !f.starts_with(&format!("{}/", last)) {
                ans.push(f);
                last = ans.last().unwrap().as_str();
            }
        }

        ans
    }
}
