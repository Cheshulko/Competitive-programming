// https://leetcode.com/problems/two-best-non-overlapping-events

class Solution {
   public:
    int maxTwoEvents(vector<vector<int>>& events) {
        sort(events.begin(), events.end(),
             [](const auto& e1, const auto& e2) { return e1[1] < e2[1]; });

        vector<int> pref_max;
        pref_max.push_back(-1);

        vector<int> pref;
        pref.push_back(-1);

        int ans = 0;
        for (const auto& ev : events) {
            ans = max(ans, ev[2]);

            auto p = upper_bound(pref.cbegin(), pref.cend(), ev[0] - 1) -
                     pref.cbegin() - 1;

            if (p != 0) {
                ans = max(ans, ev[2] + pref_max[p]);
            }

            pref_max.push_back(max(pref_max.back(), ev[2]));
            pref.push_back(ev[1]);
        }

        return ans;
    }
};