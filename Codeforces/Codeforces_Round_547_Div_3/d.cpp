#include <algorithm>
#include <iostream>
#include <utility>
#include <vector>

using namespace std;

pair<char, int> l[150001], r[150001];
int n;
char c;
vector<pair<int, int>> ans;

int main() {
    cin >> n;
    for (int i = 0; i < n; ++i) {
        cin >> c;
        l[i] = {c == '?' ? '{' : c, i + 1};
    }
    for (int i = 0; i < n; ++i) {
        cin >> c;
        r[i] = {c == '?' ? '{' : c, i + 1};
    }

    sort(l, l + n);
    sort(r, r + n);

    int i = 0, j = 0;
    int in = n, jn = n;

    while (i < in && j < jn) {
        if (l[i].first == r[j].first) {
            ans.push_back({l[i].second, r[j].second});
            ++i, ++j;
        } else if (l[i].first > r[j].first) {
            if (l[i].first == '{') {
                ans.push_back({l[i].second, r[j].second});
                ++i, ++j;
            } else {
                if (l[in - 1].first == '{') {
                    ans.push_back({l[in - 1].second, r[j].second});
                    ++j;
                    --in;
                } else {
                    ++j;
                }
            }
        } else if (l[i].first < r[j].first) {
            if (r[j].first == '{') {
                ans.push_back({l[i].second, r[j].second});
                ++i, ++j;
            } else {
                if (r[jn - 1].first == '{') {
                    ans.push_back({l[i].second, r[jn - 1].second});
                    ++i;
                    --jn;
                } else {
                    ++i;
                }
            }
        }
    }
    cout << ans.size() << endl;
    for (auto& p : ans) {
        cout << p.first << " " << p.second << endl;
    }

    return 0;
}