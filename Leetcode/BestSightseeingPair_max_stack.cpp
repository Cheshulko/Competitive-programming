// https://leetcode.com/problems/best-sightseeing-pair

class Solution {
   public:
    int maxScoreSightseeingPair(vector<int>& values) {
        const int n = values.size();

        stack<int> maxStack;
        for (int i = n - 1; i >= 0; --i) {
            if (maxStack.empty()) {
                maxStack.push(values[i] - i);
            } else {
                maxStack.push(max(int(values[i] - i), maxStack.top()));
            }
        }

        auto ans = -1;
        auto curMax = -1;
        for (auto i = 0; i < n - 1; ++i) {
            curMax = max(curMax, int(values[i] + i));
            maxStack.pop();

            ans = max(ans, curMax + maxStack.top());
        }

        return ans;
    }
};