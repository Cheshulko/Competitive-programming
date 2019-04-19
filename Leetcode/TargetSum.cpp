class Solution {
   public:
    int findTargetSumWays(vector<int>& nums, int S) {
        if (nums.empty()) return S == 0;
        int mx = 2000;
        if (S > mx) return 0;

        vector<vector<int>> dp(nums.size(), vector<int>(4002));
        dp[0][mx + nums[0]]++;
        dp[0][mx - nums[0]]++;

        for (int i = 1; i < nums.size(); ++i) {
            for (int k = nums[i]; k <= 2 * mx - nums[i]; ++k) {
                dp[i][k] += dp[i - 1][k - nums[i]];
                dp[i][k] += dp[i - 1][k + nums[i]];
            }
        }

        return dp[nums.size() - 1][mx + S];
    }
};