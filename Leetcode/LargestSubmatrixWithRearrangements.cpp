// https://leetcode.com/problems/largest-submatrix-with-rearrangements

class Solution {
   public:
    int largestSubmatrix(vector<vector<int>>& matrix) {
        const auto n = matrix.size();
        const auto m = matrix[0].size();

        vector<set<pair<int, int>>> rows(n, set<pair<int, int>>());
        for (int j = 0; j < m; ++j) {
            int cnt = 0;
            for (int i = n - 1; i >= 0; --i) {
                if (matrix[i][j]) {
                    cnt += 1;
                } else {
                    cnt = 0;
                }

                rows[i].insert({-cnt, j});
            }
        }

        int ans = 0;
        for (int i = 0; i < n; ++i) {
            int prev = numeric_limits<int>::max();
            int j = 1;
            for (const auto& [cnt, _] : rows[i]) {
                prev = min(prev, -cnt);
                ans = max(ans, prev * j);
                ++j;
            }
        }

        return ans;
    }
};