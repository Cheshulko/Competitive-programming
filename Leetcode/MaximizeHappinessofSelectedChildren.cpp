// https://leetcode.com/problems/maximize-happiness-of-selected-children

class Solution {
   public:
    long long maximumHappinessSum(vector<int>& happiness, int k) {
        sort(happiness.begin(), happiness.end());

        long long ans = 0;
        for (int i = 0; i < k; ++i) {
            const auto last = happiness[happiness.size() - 1 - i];
            ans += max(last - i, 0);
        }

        return ans;
    }
};