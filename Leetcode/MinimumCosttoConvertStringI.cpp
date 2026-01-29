// https://leetcode.com/problems/minimum-cost-to-convert-string-i

class Solution {
   public:
    long long minimumCost(string source,
                          string target,
                          vector<char>& original,
                          vector<char>& changed,
                          vector<int>& cost) {
        const int NS = 26;
        const auto INF = numeric_limits<int>::max();

        vector<vector<int>> mtx(NS, vector<int>(NS, INF));

        for (int i = 0; i < original.size(); ++i) {
            const auto u = original[i] - 'a';
            const auto v = changed[i] - 'a';

            mtx[u][v] = min(mtx[u][v], cost[i]);
        }

        for (int o = 0; o < NS; ++o) {
            for (int u = 0; u < NS; ++u) {
                for (int v = 0; v < NS; ++v) {
                    for (int k = 0; k < NS; ++k) {
                        auto& uv = mtx[u][v];
                        const auto& uk = mtx[u][k];
                        const auto& kv = mtx[k][v];

                        if (uk != INF && kv != INF && uk + kv < uv) {
                            uv = uk + kv;
                        }
                    }
                }
            }
        }

        long long ans = 0;
        for (int i = 0; i < source.size(); ++i) {
            const auto u = source[i] - 'a';
            const auto v = target[i] - 'a';

            if (u == v) {
                continue;
            }

            if (mtx[u][v] == INF) {
                return -1;
            }

            ans += mtx[u][v];
        }

        return ans;
    }
};