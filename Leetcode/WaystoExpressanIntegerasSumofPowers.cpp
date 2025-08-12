// https://leetcode.com/problems/ways-to-express-an-integer-as-sum-of-powers

class Solution {
   public:
    int numberOfWays(int n, int x) {
        using i64 = long long;
        const i64 MOD = 1000000000 + 7;

        vector<i64> nums;
        for (i64 i = 1; i <= n; ++i) {
            auto curi = i;
            for (i64 j = 1; j < x; ++j) {
                curi *= i;
            }
            nums.push_back(curi);
        }

        const auto m = nums.size();
        vector<vector<i64>> dp(n + 1, vector<i64>(m + 1, 0));

        dp[0][0] = 1;
        for (auto i = 1; i <= m; ++i) {
            const auto num = nums[i - 1];

            for (auto j = 0; j <= n; ++j) {
                dp[j][i] += dp[j][i - 1];
                dp[j][i] %= MOD;

                if (j + num <= n) {
                    dp[j + num][i] += dp[j][i - 1];
                    dp[j + num][i] %= MOD;
                }
            }
        }

        return dp[n][m];
    }
};