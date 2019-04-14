#include <stdio.h>
#include <stdlib.h>
#include <vector>
using namespace std;

bool used[20][20];
int n, m;
vector<pair<int, int>> ans;
int cnt;
int T, a, b;

bool dfs(int x, int y) {
    used[x][y] = true;
    ++cnt;

    if (cnt == n * m) {
        return true;
    }

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            if (used[i][j]) continue;
            if (x == i || y == j || x - y == i - j || x + y == i + j) continue;

            if (dfs(i, j)) {
                ans.push_back({i, j});
                return true;
            }
        }
    }

    used[x][y] = false;
    --cnt;
    return false;
}

int main() {
    scanf("%d", &T);

    for (int t = 0; t < T; ++t) {
        scanf("%d %d", &n, &m);
        memset(used, 0, sizeof used);
        ans.clear();
        cnt = 0;

        bool done = false;

        for (int i = 0; i < n && !done; ++i) {
            for (int j = 0; j < m && !done; ++j) {
                if (dfs(i, j)) {
                    ans.push_back({i, j});
                    done = true;
                }
            }
        }

        if (done) {
            printf("Case #%d: POSSIBLE\n", t + 1);
            for (auto an : ans) {
                printf("%d %d\n", an.first + 1, an.second + 1);
            }
        } else {
            printf("Case #%d: IMPOSSIBLE\n", t + 1);
        }
    }

    return 0;
}