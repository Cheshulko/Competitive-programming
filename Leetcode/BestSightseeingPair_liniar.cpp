// https://leetcode.com/problems/best-sightseeing-pair

class Solution {
   public:
    int maxScoreSightseeingPair(vector<int>& values) {
        const int n = values.size();

        auto start = values[0];
        auto ans = -1;
        for (int i = 1; i < n; ++i) {
            ans = max(ans, start + int(values[i] - i));
            start = max(start, int(values[i] + i));
        }

        return ans;
    }
};