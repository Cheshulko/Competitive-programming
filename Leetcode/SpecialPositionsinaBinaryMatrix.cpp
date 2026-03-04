// https://leetcode.com/problems/special-positions-in-a-binary-matrix

class Solution {
   public:
    int numSpecial(vector<vector<int>>& mat) {
        const auto n = mat.size();
        const auto m = mat[0].size();

        vector<int> row(n, 0), col(m, 0);

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                row[i] += (mat[i][j] == 0);
            }
        }

        for (int j = 0; j < m; ++j) {
            for (int i = 0; i < n; ++i) {
                col[j] += (mat[i][j] == 0);
            }
        }

        int ans = 0;
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (mat[i][j] && row[i] == m - 1 && col[j] == n - 1) {
                    ans += 1;
                }
            }
        }

        return ans;
    }
};