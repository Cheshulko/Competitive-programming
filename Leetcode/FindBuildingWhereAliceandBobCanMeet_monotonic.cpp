// https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet/description

class Solution {
   public:
    vector<int> leftmostBuildingQueries(vector<int>& heights,
                                        vector<vector<int>>& queries) {
        auto n = heights.size();
        auto qn = queries.size();
        vector<int> ans(qn, -1);

        vector<pair<int, int>> pj_and_ind;
        for (auto i = 0; i < qn; ++i) {
            auto& pi = queries[i][0];
            auto& pj = queries[i][1];

            if (pi > pj) {
                swap(pi, pj);
            }

            if (pi == pj || heights[pj] > heights[pi]) {
                ans[i] = pj;
            } else {
                pj_and_ind.push_back({pj, i});
            }
        }

        sort(pj_and_ind.begin(), pj_and_ind.end(),
             std::greater<pair<int, int>>());

        auto cur_ind = n - 1;
        vector<pair<int, int>> monotonic;
        for (const auto [_, ind] : pj_and_ind) {
            auto pi = queries[ind][0];
            auto pj = queries[ind][1];

            for (; cur_ind > pj; --cur_ind) {
                while (!monotonic.empty() &&
                       monotonic.back().first < heights[cur_ind]) {
                    monotonic.pop_back();
                }
                monotonic.push_back({heights[cur_ind], cur_ind});
            }

            const auto next_ind =
                upper_bound(monotonic.rbegin(), monotonic.rend(),
                            make_pair(heights[pi], n + 1));

            ans[ind] = next_ind == monotonic.rend() ? -1 : next_ind->second;
        }

        return ans;
    }
};