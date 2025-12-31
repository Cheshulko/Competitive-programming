// https://leetcode.com/problems/last-day-where-you-can-still-cross

class Solution {
   public:
    bool can(int row, int col, int day, vector<vector<int>>& cells) {
        vector<vector<int>> grid(row, vector<int>(col, 0));

        for (int i = 0; i < day; ++i) {
            grid[cells[i][0] - 1][cells[i][1] - 1] = 1;
        }

        deque<pair<int, int>> q;
        for (int j = 0; j < col; ++j) {
            if (grid[0][j] == 0) {
                grid[0][j] = 2;
                q.push_back({0, j});
            }
        }

        while (!q.empty()) {
            const auto [i, j] = q.front();
            q.pop_front();

            if (i == row - 1) {
                return true;
            }

            for (int di = -1; di <= 1; ++di) {
                for (int dj = -1; dj <= 1; ++dj) {
                    if (abs(di + dj) != 1) {
                        continue;
                    }

                    int to_i = i + di;
                    int to_j = j + dj;
                    if (to_i < 0 || to_i >= row || to_j < 0 || to_j >= col) {
                        continue;
                    }

                    if (grid[to_i][to_j] > 0) {
                        continue;
                    }

                    grid[to_i][to_j] = 2;
                    q.push_back({to_i, to_j});
                }
            }
        }

        return false;
    }

    int latestDayToCross(int row, int col, vector<vector<int>>& cells) {
        int l = 0;
        int r = cells.size();

        while (r - l > 1) {
            int m = (l + r) >> 1;
            if (can(row, col, m, cells)) {
                l = m;
            } else {
                r = m;
            }
        }

        return l;
    }
};