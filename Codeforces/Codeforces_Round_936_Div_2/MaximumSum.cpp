#include <algorithm>
#include <cmath>
#include <iostream>
#include <queue>
#include <set>
#include <stack>
#include <unordered_set>
#include <vector>

using namespace std;

long long mod = 1000000000 + 7;
long long mod_m = mod * 2 * 100001;

int main() {
    int t;
    cin >> t;

    for (int t_ = 0; t_ < t; ++t_) {
        long long n, k;
        cin >> n >> k;

        long long mx = 0;
        long long d = 0;
        long long sum = 0;
        long long x;
        for (int i = 0; i < n; ++i) {
            cin >> x;
            sum += x;
            d += x;
            d = max((long long)0, d);
            mx = max(d, mx);
        }

        for (int i = 0; i < k; ++i) {
            sum = (sum + mx + mod_m) % mod;
            mx = (2 * mx) % mod;
        }

        cout << sum << endl;
    }
}