// https://leetcode.com/problems/shortest-common-supersequence

struct Solution {}

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        fn sovle(
            i: usize,
            j: usize,
            str1: &Vec<char>,
            str2: &Vec<char>,
            dp: &mut Vec<Vec<usize>>,
            ans: &mut Vec<char>,
            cur: &mut Vec<char>,
        ) {
            if cur.len() >= dp[i][j] {
                return;
            }

            if i == str1.len() && j == str2.len() {
                if cur.len() < dp[i][j] {
                    dp[i][j] = cur.len();
                    *ans = cur.clone();
                }

                return;
            }

            dp[i][j] = cur.len();

            if i + 1 <= str1.len() && j + 1 <= str2.len() && str1[i] == str2[j] {
                cur.push(str1[i]);
                sovle(i + 1, j + 1, str1, str2, dp, ans, cur);
                cur.pop();
            } else {
                if i + 1 <= str1.len() {
                    cur.push(str1[i]);
                    sovle(i + 1, j, str1, str2, dp, ans, cur);
                    cur.pop();
                }

                if j + 1 <= str2.len() {
                    cur.push(str2[j]);
                    sovle(i, j + 1, str1, str2, dp, ans, cur);
                    cur.pop();
                }
            }
        }

        let str1 = str1.chars().collect::<Vec<_>>();
        let str2 = str2.chars().collect::<Vec<_>>();

        let mut dp = vec![vec![usize::MAX; str2.len() + 2]; str1.len() + 2];
        let mut cur = vec![];
        let mut ans = vec![];
        sovle(0, 0, &str1, &str2, &mut dp, &mut ans, &mut cur);

        ans.into_iter().collect()
    }
}
