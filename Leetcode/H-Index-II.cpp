// https://leetcode.com/problems/h-index-ii

class Solution {
   public:
    int hIndex(vector<int>& citations) {
        auto mx = *max_element(citations.cbegin(), citations.cend());
        vector<int> cnt(mx + 1);
        for (const auto& v : citations) {
            cnt[v]++;
        }
        int up = 0;
        for (int i = mx; i >= 0; --i) {
            auto inc = cnt[i];
            cnt[i] += up;
            up += inc;
        }
        int ans = 0;
        for (int i = 0; i <= mx; ++i) {
            ans = max(ans, cnt[i] >= i ? i : 0);
        }

        return ans;
    }
};