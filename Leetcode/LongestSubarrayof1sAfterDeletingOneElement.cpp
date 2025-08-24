// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element

class Solution {
   public:
    int longestSubarray(vector<int>& nums) {
        vector<int> lens;
        int cnt = 0;
        int prev = nums[0];

        for (const int& num : nums) {
            if (num == prev) {
                cnt += 1;
            } else {
                lens.push_back(cnt);
                cnt = 1;
                prev = num;
            }
        }
        lens.push_back(cnt);

        if (lens.size() == 1) {
            return nums[0] ? lens[0] - 1 : 0;
        }

        int ans = 0;
        int i = nums[0];
        for (int j = !i; j < lens.size(); j += 2) {
            ans = max(ans, lens[j]);
        }

        for (; i < lens.size(); i += 2) {
            if (i > 0 && i + 1 < lens.size() && lens[i] == 1) {
                ans = max(ans, lens[i - 1] + lens[i + 1]);
            }
        }
        return ans;
    }
};