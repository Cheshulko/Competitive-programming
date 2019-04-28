#include <iostream>
#include <limits>
using namespace std;

using ll = long long;

ll H, n;
ll d[2 * 100000 + 1];

int main() {
    cin >> H >> n;
    ll de = 0;
    ll need = std::numeric_limits<ll>::max();
    int sec = -1;
    for (int i = 0; i < n; ++i) {
        cin >> d[i];
        de += d[i];
        if (H + de < need) {
            sec = i;
            need = H + de;
        }
        if (H + de <= 0) {
            cout << i + 1 << endl;
            return 0;
        }
    }
    if (de >= 0) {
        cout << -1 << endl;
        return 0;
    }

    de *= -1;
    ll ans = (need / de) + (need % de > 0);
    H -= ans * de;
    ans = ans * n;

    for (int i = 0; i < n; ++i) {
        ++ans;
        H += d[i];
        if (H <= 0) {
            cout << ans << endl;
            return 0;
        }
    }

    return 0;
}