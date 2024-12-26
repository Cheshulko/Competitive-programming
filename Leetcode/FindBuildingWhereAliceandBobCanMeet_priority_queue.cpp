// https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet

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
        sort(pj_and_ind.begin(), pj_and_ind.end());

        auto pj_ind = 0;
        priority_queue<pair<int, int>, vector<pair<int, int>>,
                       greater<pair<int, int>>>
            min_priority_queue;
        for (auto cur_ind = 0; cur_ind < n; ++cur_ind) {
            while (pj_ind < pj_and_ind.size() &&
                   pj_and_ind[pj_ind].first <= cur_ind) {
                auto ind = pj_and_ind[pj_ind].second;
                auto pi = queries[ind][0];
                min_priority_queue.push({heights[pi], ind});

                pj_ind += 1;
            }

            while (!min_priority_queue.empty() &&
                   min_priority_queue.top().first < heights[cur_ind]) {
                auto [_, ind] = min_priority_queue.top();
                min_priority_queue.pop();

                ans[ind] = cur_ind;
            }
        }

        return ans;
    }
};