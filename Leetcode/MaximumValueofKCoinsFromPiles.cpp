// https://leetcode.com/problems/maximum-value-of-k-coins-from-piles

class Solution {
   public:
    int maxValueOfCoins(vector<vector<int>>& piles, int k) {
        int n = piles.size();
        vector<vector<long long>> pref(n, vector<long long>());
        for (int i = 0; i < n; ++i) {
            if (!piles[i].empty()) {
                pref[i].resize(piles[i].size() + 1);
                for (int j = 0; j < piles[i].size(); ++j) {
                    pref[i][j + 1] = pref[i][j] + piles[i][j];
                }
            }
        }

        vector<long long> dp(k + 1, 0);

        for (int i = 0; i < n; ++i) {
            for (int j = k; j >= 1; --j) {
                long long mx = dp[j];
                for (int len = 1; len <= j && len <= piles[i].size(); ++len) {
                    mx = max(mx, dp[j - len] + pref[i][len]);
                }
                dp[j] = mx;
            }
        }

        return dp[k];
    }
};