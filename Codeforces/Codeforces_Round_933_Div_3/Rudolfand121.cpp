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

int main() {
    cin.tie(0)->sync_with_stdio(0);
    cin.exceptions(cin.failbit);

    cin >> t;
    for (auto _t = 0; _t < t; ++_t) {
        string s;
        cin >> n;

        vector<int> a(n, 0);
        for (int i = 0; i < n; ++i) {
            cin >> a[i];
        }

        auto can = true;
        for (int i = 1; i < n - 1; ++i) {
            auto need = a[i - 1];
            if (a[i] >= 2 * need && a[i + 1] >= need) {
                a[i - 1] -= need;
                a[i] -= 2 * need;
                a[i + 1] -= need;
            } else {
                can = false;
                break;
            }
        }

        can &= a[n - 1] == 0 && a[n - 2] == 0;
        cout << (can ? "Yes" : "No") << endl;
    }
}