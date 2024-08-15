// https://leetcode.com/problems/lemonade-change

class Solution {
   public:
    bool lemonadeChange(vector<int>& bills) {
        vector<int> cnt(21, 0);
        for (const auto& b : bills) {
            cnt[b]++;
            auto c = b - 5;
            while (c >= 10 && cnt[10]) {
                c -= 10;
                cnt[10]--;
            }
            while (c >= 5 && cnt[5]) {
                c -= 5;
                cnt[5]--;
            }
            if (c != 0) {
                return false;
            }
        }
        return true;
    }
};