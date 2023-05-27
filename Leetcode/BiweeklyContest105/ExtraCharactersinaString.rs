impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut dp = vec![usize::MAX; s.len() + 1];

        fn go(s: &str, dictionary: &Vec<String>, dp: &mut Vec<usize>) -> usize {
            if s.len() == 0 {
                return 0;
            }
            if dp[s.len()] != usize::MAX {
                return dp[s.len()];
            }

            let mut ans = s.len();

            for cur in 0..s.len() {
                let str = s.get(0..=cur).unwrap().to_owned();
                if dictionary.contains(&str) {
                    ans = ans.min(go(s.get((cur + 1)..).unwrap(), dictionary, dp))
                }
            }
            ans = ans.min(1 + go(s.get(1..).unwrap(), dictionary, dp));

            dp[s.len()] = ans;
            return ans;
        }

        go(s.as_str(), &dictionary, &mut dp) as i32
    }
}
