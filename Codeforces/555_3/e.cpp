#include <algorithm>
#include <iostream>
#include <map>

using namespace std;

const int N = 2 * 100000 + 1;

int n, x;
int a[N];
int b[N];
int c[N];

map<int, int> cnt;

void build(int i, std::map<int, int>::iterator &it) {
    int elem = it->first;
    c[i] = elem;
    it->second--;
    if (it->second == 0) {
        cnt.erase(it);
    }
}

int main() {
    cin >> n;
    for (int i = 0; i < n; ++i) cin >> a[i];
    for (int i = 0; i < n; ++i) {
        cin >> b[i];
        cnt[b[i]]++;
    }
    for (int i = 0; i < n; ++i) {
        int op = n - a[i];
        auto p = cnt.upper_bound(op - 1);
        if (p != cnt.end()) {
            build(i, p);
        } else {
            auto p = cnt.upper_bound(-1);
            build(i, p);
        }
    }
    for (int i = 0; i < n; ++i) {
        cout << (a[i] + c[i]) % n << " ";
    }
    cout << endl;

    return 0;
}