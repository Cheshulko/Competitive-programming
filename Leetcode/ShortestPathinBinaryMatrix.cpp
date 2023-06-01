// https://leetcode.com/problems/shortest-path-in-binary-matrix

class Solution {
   public:
    int shortestPathBinaryMatrix(vector<vector<int>>& grid) {
        int n = grid.size();

        queue<tuple<int, int, int>> v;
        vector<vector<bool>> used(101, vector<bool>(101, false));

        if (grid[0][0] == 0) {
            v.push(make_tuple(0, 0, 1));
            used[0][0] = true;
        }

        int ans = -1;

        while (!v.empty()) {
            int i, j, dist;
            tie(i, j, dist) = v.front();
            v.pop();

            if (i == n - 1 && j == n - 1) {
                ans = dist;
                break;
            }

            int di[] = {0, 1, 1, 1, 0, -1, -1, -1};
            int dj[] = {1, -1, 0, 1, -1, 1, 0, -1};

            for (int k = 0; k < 8; k++) {
                int to_i = i + di[k];
                int to_j = j + dj[k];
                if (to_i >= 0 && to_i < n && to_j >= 0 && to_j < n &&
                    !used[to_i][to_j] && grid[to_i][to_j] == 0) {
                    v.push(make_tuple(to_i, to_j, dist + 1));
                    used[to_i][to_j] = true;
                }
            }
        }

        return ans;
    }
};