impl Solution {
    pub fn get_words_in_longest_subsequence(
        n: i32,
        words: Vec<String>,
        groups: Vec<i32>,
    ) -> Vec<String> {
        let n = n as usize;
        let mut ans = vec![];
        let mut cur = groups[0];

        ans.push(words[0].clone());

        for i in 1..n {
            if cur != groups[i] {
                cur = groups[i];
                ans.push(words[i].clone());
            }
        }

        ans
    }
}
