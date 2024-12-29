// https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary

class Solution {
   public:
    int numWays(vector<string>& words, string target) {
        using i64 = long long;

        const i64 MOD = 1e9 + 7;
        const int N = target.length();
        const int K = words.size();
        const int W = words[0].length();

        vector<array<i64, 28>> count(W, array<i64, 28>());
        for (int wPos = 0; wPos < W; ++wPos) {
            for (int wInd = 0; wInd < K; ++wInd) {
                ++count[wPos][words[wInd][wPos] - 'a'];
            }
        }

        vector<vector<i64>> dp(N + 1, vector<i64>(W + 1, 0));
        for (int wInd = 0; wInd <= W; ++wInd) {
            dp[0][wInd] = 1;
        }

        for (int tLen = 1; tLen <= N; ++tLen) {
            for (int wLen = 1; wLen <= W; ++wLen) {
                dp[tLen][wLen] = dp[tLen][wLen - 1] +
                                 dp[tLen - 1][wLen - 1] *
                                     count[wLen - 1][target[tLen - 1] - 'a'];
                dp[tLen][wLen] %= MOD;
            }
        }

        return dp[N][W];
    }
};