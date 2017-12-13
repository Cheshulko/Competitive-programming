class Solution {
public:
    int maxProduct(vector<int>& nums) {
        int l = nums.size();
        pair<int, int> *dp = new pair<int, int>[l];
        dp[0].first = dp[0].second = nums[0];
        for(int i = 1; i < l; ++i){
            dp[i].first = max(nums[i], max(dp[i-1].first * nums[i], dp[i-1].second * nums[i]));
            dp[i].second = min(nums[i], min(dp[i-1].first * nums[i], dp[i-1].second * nums[i]));
        }
        int ans = nums[0];
        for(int i = 1; i < l; ++i) ans = max(ans, max(dp[i].first, dp[i].second));
        return ans;
    }
};
