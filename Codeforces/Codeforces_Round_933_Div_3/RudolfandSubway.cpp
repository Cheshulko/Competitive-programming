#include <algorithm>
#include <cmath>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <stack>
#include <unordered_set>
#include <vector>

using namespace std;

const int INF = 1000000000;
const int MAX = 2 * 100000 + 1;
int t;
int n, m;
int u, v, c;
int s, e;

int main() {
    cin.tie(0)->sync_with_stdio(0);
    cin.exceptions(cin.failbit);

    cin >> t;
    for (auto _t = 0; _t < t; ++_t) {
        map<int, int> cn;
        vector<pair<pair<int, int>, int>> edges;
        cin >> n >> m;

        for (auto i = 0; i < m; ++i) {
            cin >> u >> v >> c;
            --v, --u;
            edges.push_back({{u, v}, c});
            const auto ind = cn.size();
            if (cn.find(c) == cn.cend()) {
                cn[c] = ind;
            }
        }

        auto l = cn.size();
        vector<vector<int>> adj(n + l, vector<int>());
        for (const auto& edge : edges) {
            const auto c = cn[edge.second];
            const auto [v, u] = edge.first;
            adj[v].push_back(n + c);
            adj[u].push_back(n + c);
            adj[n + c].push_back(v);
            adj[n + c].push_back(u);
        }

        cin >> s >> e;
        --s, --e;

        vector<int> visited(n + l, false);
        queue<pair<int, int>> q;
        q.push({s, 0});
        visited[s] = true;

        auto ans = 0;
        while (!q.empty()) {
            const auto [v, d] = q.front();
            q.pop();

            if (v == e) {
                ans = d / 2;
                break;
            }

            for (const auto& to : adj[v]) {
                if (!visited[to]) {
                    visited[to] = true;
                    q.push({to, d + 1});
                }
            }
        }

        cout << ans << endl;
    }
}