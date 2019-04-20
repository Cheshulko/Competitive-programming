#include <stdio.h>
#include <algorithm>
#include <bitset>
#include <cmath>
#include <ctime>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

#define Work_It_Harder_Make_It_Better_Do_It_Faster_Makes_Us_Stronger \
    ios::sync_with_stdio(0);                                         \
    cin.tie(0);                                                      \
    cout.tie();

int T;
int n;

int main() {
    cin >> T;
    for (int t = 0; t < T; ++t) {
        cin >> n;

        int l = 2, r = n;
        int mx = -2;
        int x = -1;

        cout << 1 << " " << (r - l + 1) << " " << 1 << " ";
        for (int i = l; i <= r; ++i) {
            cout << i << " ";
        }
        cout << endl;
        cout.flush();

        cin >> mx;

        while (r - l > 1) {
            int mid = (l + r) / 2 + (l + r) % 2;
            cout << 1 << " " << (r - mid + 1) << " " << 1 << " ";
            for (int i = mid; i <= r; ++i) {
                cout << i << " ";
            }
            cout << endl;
            cout.flush();

            cin >> x;
            if (x >= mx) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }

        int gg;

        if (r - l == 1) {
            cout << "1 1 1 " << r << endl;
            cout.flush();

            cin >> x;
            if (x >= mx) {
                gg = r;
            } else {
                gg = l;
            }
        } else {
            gg = r;
        }

        cout << 1 << " " << (n - 1) << " " << gg << " ";
        for (int i = 1; i <= n; ++i) {
            if (i == gg) continue;
            cout << i << " ";
        }
        cout << endl;
        cout.flush();

        int ans = -1;
        cin >> ans;

        cout << -1 << " " << ans << endl;
        cout.flush();
    }

    return 0;
}