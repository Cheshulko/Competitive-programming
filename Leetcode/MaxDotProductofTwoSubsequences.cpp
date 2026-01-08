// https://leetcode.com/problems/max-dot-product-of-two-subsequences

class Solution {
   public:
    int maxDotProduct(vector<int>& nums1, vector<int>& nums2) {
        auto n1 = nums1.size();
        auto n2 = nums2.size();

        vector<vector<int>> dp(
            n1 + 1,
            vector<int>(n2 + 1, 1000000 + 1 + numeric_limits<int>::min()));

        for (size_t i = 0; i < n1; ++i) {
            for (size_t j = 0; j < n2; ++j) {
                dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j]);
                dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i + 1][j]);
                dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j + 1]);
                dp[i + 1][j + 1] =
                    max(dp[i + 1][j + 1], dp[i][j] + nums1[i] * nums2[j]);
                dp[i + 1][j + 1] = max(dp[i + 1][j + 1], nums1[i] * nums2[j]);
            }
        }

        return dp[n1][n2];
    }
};