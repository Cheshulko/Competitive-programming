// https://leetcode.com/problems/count-unguarded-cells-in-the-grid

class Solution {
   public:
    int countUnguarded(int n,
                       int m,
                       vector<vector<int>>& guards,
                       vector<vector<int>>& walls) {
        using cell = pair<int, int>;

        enum CellType { Guard = 0, Wall = 1, Fake = 42 };

        constexpr auto ma = numeric_limits<int>::max();
        constexpr auto mi = numeric_limits<int>::min();

        vector<set<cell>> rows(n);
        vector<set<cell>> cols(m);
        for (auto i = 0; i < n; ++i) {
            rows[i].insert({mi, CellType::Fake});
            rows[i].insert({ma, CellType::Fake});
        }
        for (auto j = 0; j < m; ++j) {
            cols[j].insert({mi, CellType::Fake});
            cols[j].insert({ma, CellType::Fake});
        }

        for (const auto& guard : guards) {
            const auto row = guard[0];
            const auto col = guard[1];

            rows[row].insert({col, CellType::Guard});
            cols[col].insert({row, CellType::Guard});
        }
        for (const auto& wall : walls) {
            const auto row = wall[0];
            const auto col = wall[1];

            rows[row].insert({col, CellType::Wall});
            cols[col].insert({row, CellType::Wall});
        }

        auto ans = 0;
        for (auto i = 0; i < n; ++i) {
            for (auto j = 0; j < m; ++j) {
                auto col = rows[i].lower_bound({j, mi});
                if (col->first == j) {
                    continue;
                }

                auto row = cols[j].lower_bound({i, mi});
                if (col->second != CellType::Guard &&
                    (--col)->second != CellType::Guard &&
                    row->second != CellType::Guard &&
                    (--row)->second != CellType::Guard) {
                    ++ans;
                }
            }
        }

        return ans;
    }
};