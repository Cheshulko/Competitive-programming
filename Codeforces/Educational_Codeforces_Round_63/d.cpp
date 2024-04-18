#include <algorithm>
#include <iostream>

using namespace std;

const int N = 3 * 100000 + 1;
const long long Z = 0;
int n, x;
long long a[N];

long long dp[N][2][2];

int main() {
    cin >> n >> x;
    for (int i = 1; i <= n; ++i) {
        cin >> a[i];
    }

    long long ans = 0;

    for (int i = 1; i <= n; ++i) {
        dp[i][0][0] = 
            max({Z, dp[i - 1][0][0] + a[i]});
        dp[i][1][0] =
            max({Z, dp[i - 1][0][0] + x * a[i], dp[i - 1][1][0] + x * a[i]});
        dp[i][0][1] = 
            max({Z, dp[i - 1][0][1] + a[i], dp[i - 1][1][0] + a[i]});

        ans = max({ans, dp[i][0][0], dp[i][1][0], dp[i][0][1]});
    }

    cout << ans << endl;
    return 0;
}