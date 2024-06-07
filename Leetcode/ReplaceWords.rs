// https://leetcode.com/problems/replace-words

struct Solution {}

impl Solution {
    pub fn replace_words(mut dictionary: Vec<String>, sentence: String) -> String {
        dictionary.sort_by_key(|x| x.len());

        let mut ans = vec![];
        for s in sentence.split(' ') {
            let mut ok = false;
            for d in dictionary.iter() {
                if s.len() >= d.len() && &s[..d.len()] == d.as_str() {
                    ans.push(d.clone());
                    ok = true;
                    break;
                }
            }
            if !ok {
                ans.push(s.to_string());
            }
        }

        ans.join(" ")
    }
}
