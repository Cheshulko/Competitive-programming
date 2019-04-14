#include <stdio.h>
#include <stdlib.h>
#include <vector>
using namespace std;

int T;
int n, m;
vector<pair<int, int>> ans;

void fill2(int x0, int y0, int len) {
    for (int k = 0; k < len; ++k) {
        ans.push_back({y0, x0});
        ans.push_back({y0 + 1, (x0 + 3) % len});
        x0 = (x0 + 1) % len;
    }
}

void fill3(int x0, int y0, int len) {
    for (int k = 0; k < len; ++k) {
        ans.push_back({y0 - 1, x0});
        ans.push_back({y0 - 2, (x0 + 2) % len});
        ans.push_back({y0, (x0 + 3) % len});
        x0 = (x0 + 1) % len;
    }
}

inline void sw() {
    for (auto& p : ans) {
        swap(p.first, p.second);
    }
}

vector<pair<int, int>> hackSorry = {
    {4, 4}, {2, 1}, {3, 3}, {4, 1}, {2, 2}, {4, 3}, {1, 4}, {4, 2},
    {3, 4}, {1, 3}, {3, 2}, {2, 4}, {1, 2}, {3, 1}, {2, 3}, {1, 1}};

int main() {
    scanf("%d", &T);

    for (int t = 0; t < T; ++t) {
        scanf("%d %d", &n, &m);
        ans.clear();
        int mn = min(n, m);
        int mx = max(n, m);

        if (mx <= 3 || (mx == 4 && mn == 2)) {
            printf("Case #%d: IMPOSSIBLE\n", t + 1);
            continue;
        }

        if (n == 4 && m == 4) {
            ans = hackSorry;
            printf("Case #%d: POSSIBLE\n", t + 1);
            for (const auto& p : ans) {
                printf("%d %d\n", p.first, p.second);
            }
            continue;
        }

        bool need_sw = (m < n);
        if (need_sw) swap(n, m);

        int i = 0;
        if (n % 2) {
            i += 3;
            fill3(0, 2, m);
        }

        for (; i < n; i += 2) {
            fill2(0, i, m);
        }

        if (need_sw) sw();

        printf("Case #%d: POSSIBLE\n", t + 1);
        int cnt = 1;
        for (const auto& p : ans) {
            printf("%d %d\n", p.first + 1, p.second + 1);
        }
    }

    return 0;
}
