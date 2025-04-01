// https://leetcode.com/problems/solving-questions-with-brainpower

class Solution {
   public:
    long long mostPoints(vector<vector<int>>& questions) {
        const auto n = questions.size();

        vector<long long> dp(n, 0);
        long long ans = 0;

        for (auto i = 0; i < n; ++i) {
            if (1 + i < n) {
                dp[1 + i] = max(dp[1 + i], dp[i]);
            }

            dp[i] += questions[i][0];

            ans = max(ans, dp[i]);

            if (1 + i + questions[i][1] < n) {
                dp[1 + i + questions[i][1]] =
                    max(dp[1 + i + questions[i][1]], dp[i]);
            }
        }

        return ans;
    }
};