#include <algorithm>
#include <cmath>
#include <iostream>
#include <queue>
#include <set>
#include <stack>
#include <unordered_set>
#include <vector>

using namespace std;

int main() {
    int t;
    cin >> t;

    for (int t_ = 0; t_ < t; ++t_) {
        int n;
        cin >> n;

        vector<int> a(n);
        for (int i = 0; i < n; ++i) {
            cin >> a[i];
        }

        sort(a.begin(), a.end());

        int p = n / 2 + n % 2 - 1;
        int ans = 0;
        for (int i = p; i < n; ++i) {
            ans += a[i] == a[p];
        }
        cout << ans << endl;
    }
}