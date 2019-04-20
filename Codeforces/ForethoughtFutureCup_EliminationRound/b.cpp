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

    string s1;
    cin >> s1;

    string se = "";
    string see = "";

    for (int i = 0; i < s1.length(); ++i) {
        se.push_back(s1[i]);
        if (s1[i] == 'a') {
        } else {
            see.push_back(s1[i]);
        }
        if (se.length() + see.length() == s1.length()) {
            string s3 = se + see;
            if (s3 == s1) {
                cout << se << endl;
            } else {
                cout << ":(" << endl;
            }
            return 0;
        }
    }
    cout << ":(" << endl;

    return 0;
}