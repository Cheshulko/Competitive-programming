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

int main() {
    Work_It_Harder_Make_It_Better_Do_It_Faster_Makes_Us_Stronger;

    cin >> n >> m;
    if (n == m) {
        cout << 0 << endl;
        return 0;
    }

    cout << max(1, min(n / 2, min(n - m, m))) << endl;

    return 0;
}