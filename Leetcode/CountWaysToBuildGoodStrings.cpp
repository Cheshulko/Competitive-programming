// https://leetcode.com/problems/count-ways-to-build-good-strings

class Solution {
   public:
    const int MOD = 1e9 + 7;

    int countGoodStrings(int low, int high, int zero, int one) {
        vector<int> dp(high + 1, 0);
        dp[0] = 1;

        int ans = 0;
        for (int len = 0; len <= high; ++len) {
            if (len + zero <= high) {
                dp[len + zero] += dp[len];
                dp[len + zero] %= MOD;
            }
            if (len + one <= high) {
                dp[len + one] += dp[len];
                dp[len + one] %= MOD;
            }
            if (len >= low) {
                ans += dp[len];
                ans %= MOD;
            }
        }

        return ans;
    }
};