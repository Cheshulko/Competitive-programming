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

int n, m;
int a1[55][55], a2[55][55];

int main() {
    Work_It_Harder_Make_It_Better_Do_It_Faster_Makes_Us_Stronger;

    cin >> n >> m;
    for (int i = 0; i < n; ++i)
        for (int j = 0; j < m; ++j) cin >> a1[i][j];

    for (int i = 0; i < n; ++i)
        for (int j = 0; j < m; ++j) cin >> a2[i][j];

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            int mn = min(a1[i][j], a2[i][j]);
            int mx = max(a1[i][j], a2[i][j]);

            a1[i][j] = mn;
            a2[i][j] = mx;
        }
    }

    bool can = true;
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            if (i - 1 >= 0 && a1[i - 1][j] >= a1[i][j]) {
                can = false;
            }
            if (j - 1 >= 0 && a1[i][j - 1] >= a1[i][j]) {
                can = false;
            }

            if (i - 1 >= 0 && a2[i - 1][j] >= a2[i][j]) {
                can = false;
            }
            if (j - 1 >= 0 && a2[i][j - 1] >= a2[i][j]) {
                can = false;
            }
        }
    }

    if(can){
        cout << "Possible" << endl;
    }else{
        cout << "Impossible" << endl;
    }

    return 0;
}