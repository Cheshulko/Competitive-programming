// https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary

class Solution {
   public:
    using i64 = long long;

    const i64 MOD = 1e9 + 7;

    i64 solve(int tPos,
              int wPos,
              const vector<string>& words,
              const string& target,
              const vector<array<i64, 28>>& count,
              vector<vector<i64>>& dp) {
        const int N = target.length();
        const int W = words[0].length();

        if (tPos == N) {
            return 1;
        }

        if (wPos == W) {
            return 0;
        }

        if (std::numeric_limits<i64>::min() != dp[tPos][wPos]) {
            return dp[tPos][wPos];
        }

        dp[tPos][wPos] =
            solve(tPos, wPos + 1, words, target, count, dp) +
            count[wPos][target[tPos] - 'a'] *
                solve(tPos + 1, wPos + 1, words, target, count, dp);
        dp[tPos][wPos] %= MOD;

        return dp[tPos][wPos];
    }

    int numWays(vector<string>& words, string target) {
        const int K = words.size();
        const int N = target.length();
        const int W = words[0].length();

        vector<array<i64, 28>> count(W, array<i64, 28>());
        for (int wPos = 0; wPos < W; ++wPos) {
            for (int wInd = 0; wInd < K; ++wInd) {
                ++count[wPos][words[wInd][wPos] - 'a'];
            }
        }

        vector<vector<i64>> dp(N,
                               vector<i64>(W, std::numeric_limits<i64>::min()));

        return solve(0, 0, words, target, count, dp);
    }
};