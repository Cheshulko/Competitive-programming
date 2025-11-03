// https://leetcode.com/problems/minimum-time-to-make-rope-colorful

class Solution {
   public:
    int minCost(string colors, vector<int>& neededTime) {
        auto ans = 0;
        auto start_it = colors.begin();
        auto end_it = colors.begin();

        while (end_it != colors.cend()) {
            while (end_it != colors.cend() && *start_it == *end_it) {
                ++end_it;
            }

            auto sum = 0;
            auto ma = neededTime[start_it - colors.cbegin()];
            while (start_it != end_it) {
                const auto time = neededTime[start_it - colors.cbegin()];
                sum += time;
                if (ma < time) {
                    ma = time;
                }
                ++start_it;
            }

            ans += sum - ma;
        }

        return ans;
    }
};