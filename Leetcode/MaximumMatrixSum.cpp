// https://leetcode.com/problems/maximum-matrix-sum

class Solution {
   public:
    long long maxMatrixSum(vector<vector<int>>& matrix) {
        auto mi = numeric_limits<int>::max();
        long long ans = 0;
        size_t cnt = 0;

        for (size_t i = 0; i < matrix.size(); ++i) {
            for (size_t j = 0; j < matrix[i].size(); ++j) {
                if (matrix[i][j] < 0) {
                    cnt += 1;
                }
                ans += abs(matrix[i][j]);
                mi = min(mi, abs(matrix[i][j]));
            }
        }

        if (cnt & 1) {
            ans -= 2 * mi;
        }

        return ans;
    }
};