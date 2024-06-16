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
    cin.tie(0)->sync_with_stdio(0);
    cin.exceptions(cin.failbit);

    long long t, n, a, b;
    cin >> t;

    while (t--) {
        cin >> n >> a >> b;

        long long l = 0;
        long long r = min(n, b);

        while (r - l > 3) {
            long double d = r - l;

            long double m1 = l + d / 3.;
            long double m2 = r - d / 3.;

            long double a1 = b * m1 - (1. + m1) / 2. * m1 + m1 + (n - m1) * a;
            long double a2 = b * m2 - (1. + m2) / 2. * m2 + m2 + (n - m2) * a;

            if (a1 > a2) {
                r = m2;
            } else {
                l = m1;
            }
        }

        long long ans = 0;
        for (auto i = l; i <= min((long long)(r + 1), min(n, b)); ++i) {
            ans = max(ans, b * i + i - i * (1 + i) / 2 + (n - i) * a);
        }

        cout << ans << endl;
    }
}

// g++ -std=c++17 main.cpp -o main && ./main