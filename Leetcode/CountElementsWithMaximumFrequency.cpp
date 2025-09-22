// https://leetcode.com/problems/count-elements-with-maximum-frequency

class Solution {
   public:
    int maxFrequencyElements(vector<int>& nums) {
        array<int, 101> freq;
        auto ma = 0;

        for (const auto& num : nums) {
            auto f = freq[num] += 1;
            ma = max(ma, f);
        }

        auto ans = 0;
        for (const auto& f : freq) {
            if (f == ma) {
                ans += f;
            }
        }

        return ans;
    }
};