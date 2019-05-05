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

int n, h, m;
int x[100];

int main() {
    Work_It_Harder_Make_It_Better_Do_It_Faster_Makes_Us_Stronger;

    cin >> n >> h >> m;
    int l, r, g;
    for (int i = 0; i < n; ++i) {
        x[i] = h;
    }
    for (int i = 0; i < m; ++i) {
        cin >> l >> r >> g;
        for (int j = l - 1; j <= r - 1; ++j) {
            x[j] = min(x[j], g);
        }
    }
    int ans = 0;
    for (int i = 0; i < n; ++i) {
        ans += x[i] * x[i];
    }
    cout << ans << endl;

    return 0;
}