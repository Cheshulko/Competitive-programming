// https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and

class Solution {
   public:
    int longestSubarray(vector<int>& nums) {
        auto ma = *max_element(nums.cbegin(), nums.cend());
        auto ans = 1;

        auto cur = 0;
        for (const auto& num : nums) {
            if (num == ma) {
                cur += 1;
            } else {
                cur = 0;
            }

            ans = max(ans, cur);
        }

        return ans;
    }
};