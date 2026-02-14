// https://leetcode.com/problems/champagne-tower

class Solution {
   public:
    double champagneTower(int poured, int query_row, int query_glass) {
        vector<vector<double>> levels(100 + 1, vector<double>(100 + 1, 0));

        levels[0][0] = poured;
        for (int i = 0; i < query_row; ++i) {
            for (int j = 0; j <= i; ++j) {
                auto extra = levels[i][j] - 1.;

                if (extra > 0.) {
                    auto half = extra / 2.;
                    levels[i + 1][j] += half;
                    levels[i + 1][j + 1] += half;
                }
            }
        }

        return min(levels[query_row][query_glass], 1.);
    }
};