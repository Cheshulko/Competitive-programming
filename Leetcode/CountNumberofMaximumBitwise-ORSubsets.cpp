// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets

class Solution {
   public:
    int countMaxOrSubsets(vector<int>& nums) {
        const auto n = nums.size();

        auto ma = 0;
        for (const auto& num : nums) {
            ma |= num;
        }

        auto ans = 0;
        for (auto mask = 0; mask < (1 << n); ++mask) {
            auto cur = 0;
            for (auto bit = 0; bit < n; ++bit) {
                if (mask & (1 << bit)) {
                    cur |= nums[bit];
                }
            }
            ans += (cur == ma);
        }

        return ans;
    }
};