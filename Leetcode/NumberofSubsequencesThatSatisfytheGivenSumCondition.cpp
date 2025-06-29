// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition

class Solution {
   public:
    int numSubseq(vector<int>& nums, int target) {
        const auto MOD = 1000000000 + 7;

        sort(nums.begin(), nums.end());

        vector<int> pows(nums.size(), 0);
        for (auto i = 0, p = 1; i < nums.size(); ++i, p = (p * 2) % MOD) {
            pows[i] = p;
        }

        auto ans = 0;
        for (int i = 0; i < nums.size(); ++i) {
            auto can = target - nums[i];
            auto p = upper_bound(nums.begin(), nums.end(), can) - nums.begin();
            if (p <= i) {
                break;
            }

            ans = (ans + pows[p - i - 1]) % MOD;
        }

        return ans;
    }
};