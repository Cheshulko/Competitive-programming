#include <algorithm>
#include <cmath>
#include <iostream>
#include <queue>
#include <set>
#include <stack>
#include <unordered_set>
#include <vector>

using namespace std;

const int BITS = 30;

vector<int> a;
int n, x;

int count(int b) {
    int ans = 0;
    for (int i = 0; i < a.size(); ++i) {
        if (a[i] & (1 << b)) {
            ++i;

            while (i < a.size() && !(a[i] & (1 << b))) {
                ++i;
            }

            if (i == a.size()) {
                return -1;
            } else {
                ans += 1;
            }

        } else {
            ans += 1;
        }
    }

    return ans;
}

bool kill_bit(int b) {
    vector<int> tmp;

    for (int i = 0; i < a.size(); ++i) {
        if (a[i] & (1 << b)) {
            int acur = a[i];
            ++i;

            while (i < a.size() && !(a[i] & (1 << b))) {
                acur ^= a[i];
                ++i;
            }

            if (i == a.size()) {
                return false;
            } else {
                acur ^= a[i];
                tmp.push_back(acur);
            }

        } else {
            tmp.push_back(a[i]);
        }
    }

    swap(a, tmp);
    return true;
}

int main() {
    int t;
    cin >> t;

    while (t--) {
        cin >> n >> x;

        a.assign(n, 0);
        for (int i = 0; i < n; ++i) {
            cin >> a[i];
        }

        int high_b = 0;
        for (int b = 0; b < BITS; ++b) {
            if (x & (1 << b)) {
                high_b = b;
            }
        }

        bool remove_high_bits = true;
        for (int b = (BITS - 1); b > high_b; --b) {
            int have_bit = 0;
            for (int i = 0; i < a.size(); ++i) {
                have_bit |= (a[i] & (1 << b));
            }

            if (have_bit) {
                bool ok = kill_bit(b);

                if (!ok) {
                    cout << -1 << endl;

                    remove_high_bits = false;
                    break;
                }
            }
        }

        if (!remove_high_bits) {
            continue;
        }

        int y = 0;
        for (int i = 0; i < a.size(); ++i) {
            y |= a[i];
        }
        if (y <= x) {
            cout << a.size() << endl;
            continue;
        }

        int ans = -1;
        for (int b = high_b; b >= 0; --b) {
            if (x & (1 << b)) {
                ans = max(ans, count(b));
            } else {
                bool ok = kill_bit(b);

                if (ok) {
                    int y = 0;
                    for (int i = 0; i < a.size(); ++i) {
                        y |= a[i];
                    }
                    if (y <= x) {
                        ans = max(ans, (int)a.size());
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        cout << ans << endl;
    }
}