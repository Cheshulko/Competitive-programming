// https://leetcode.com/problems/number-of-laser-beams-in-a-bank

#include <ranges>

class Solution {
   public:
    int numberOfBeams(vector<string>& bank) {
        int last = 0;
        int ans = 0;
        for (const auto& b : bank) {
            auto ok = b | std::views::filter([](char x) { return x == '1'; });
            auto l = std::ranges::distance(ok);
            if (l > 0) {
                ans += last * l;
                last = l;
            }
        }

        return ans;
    }
};