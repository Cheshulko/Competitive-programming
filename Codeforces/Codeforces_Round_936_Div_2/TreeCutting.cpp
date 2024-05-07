#include <algorithm>
#include <cmath>
#include <iostream>
#include <queue>
#include <set>
#include <stack>
#include <unordered_set>
#include <vector>

using namespace std;

vector<vector<int>> g;
vector<bool> used;
int n, k;

int dfs(int& k, int cnt, int cur) {
    used[cur] = true;
    int cur_cnt = 1;

    for (int i = 0; i < g[cur].size(); ++i) {
        int to = g[cur][i];

        if (!used[to]) {
            int t = dfs(k, cnt, to);
            if (t >= cnt && k > 0) {
                --k;
                t = 0;
            }
            cur_cnt += t;
        }
    }

    return cur_cnt;
}

bool can(int cnt) {
    used.assign(n, false);
    int kloc = k;

    int x = dfs(kloc, cnt, 0);

    return x >= cnt && kloc == 0;
}

int main() {
    int t;
    cin >> t;

    while (t--) {
        cin >> n >> k;

        g.assign(n, vector<int>());

        int v, u;
        for (int i = 0; i < n - 1; ++i) {
            cin >> v >> u;
            --v;
            --u;
            g[v].push_back(u);
            g[u].push_back(v);
        }

        int l = 1;
        int r = n + 1;

        while (r - l > 1) {
            int m = (l + r) / 2;

            if (can(m)) {
                l = m;
            } else {
                r = m;
            }
        }

        cout << l << endl;
    }
}