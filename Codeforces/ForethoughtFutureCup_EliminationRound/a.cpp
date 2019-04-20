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

int main() {
    Work_It_Harder_Make_It_Better_Do_It_Faster_Makes_Us_Stronger;

    string s;
    cin >> s;
    int cnt = 0;
    for (char c : s) {
        if (c == 'a') {
            ++cnt;
        }
    }
    int mx = 2 * cnt - 1;
    if (s.length() < mx) mx = s.length();
    cout << mx << endl;

    return 0;
}