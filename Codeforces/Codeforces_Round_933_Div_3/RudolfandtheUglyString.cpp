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

    auto check = [&](const string& s, int i) -> bool {
        if (i + 2 >= s.length()) {
            return false;
        }
        if (s[i] == 'p' && s[i + 1] == 'i' && s[i + 2] == 'e') {
            return true;
        }
        if (s[i] == 'm' && s[i + 1] == 'a' && s[i + 2] == 'p') {
            return true;
        }
        return false;
    };

    auto check_con = [&](const string& s, int i) -> bool {
        if (i + 4 >= s.length()) {
            return false;
        }

        if (s[i] == 'm' && s[i + 1] == 'a' && s[i + 2] == 'p' &&
            s[i + 3] == 'i' && s[i + 4] == 'e') {
            return true;
        }
        return false;
    };

    cin >> t;
    for (auto _t = 0; _t < t; ++_t) {
        string s;
        cin >> n >> s;

        int ans = 0;
        for (int i = 0; i < s.length(); ++i) {
            ans += check(s, i);
            ans -= check_con(s, i);
        }

        cout << ans << endl;
    }
}