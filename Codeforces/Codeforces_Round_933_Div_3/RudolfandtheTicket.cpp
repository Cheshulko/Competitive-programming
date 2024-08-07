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
int n, m, k;

int main() {
    cin.tie(0)->sync_with_stdio(0);
    cin.exceptions(cin.failbit);

    cin >> t;
    for (auto _t = 0; _t < t; ++_t) {
        string s;
        cin >> n >> m >> k;

        vector<int> b(n, 0);
        for (int i = 0; i < n; ++i) {
            cin >> b[i];
        }
        vector<int> c(m, 0);
        for (int i = 0; i < m; ++i) {
            cin >> c[i];
        }

        sort(b.begin(), b.end());
        sort(c.begin(), c.end());

        auto ans = 0;
        for (int i = 0; i < n; ++i) {
            auto max = k - b[i];

            ans += upper_bound(c.cbegin(), c.cend(), max) - c.cbegin();
        }

        cout << ans << endl;
    }
}