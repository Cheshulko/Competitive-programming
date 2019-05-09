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

int n;
int arr[100005];
int cnt[100005];

set<int> diff;
map<int, int> m;

int mx = 0;

int main() {
    Work_It_Harder_Make_It_Better_Do_It_Faster_Makes_Us_Stronger;

    cin >> n;
    for (int i = 0; i < n; ++i) {
        cin >> arr[i];
        diff.insert(arr[i]);

        if (cnt[arr[i]] > 0) {
            m[cnt[arr[i]]]--;
            if (m[cnt[arr[i]]] == 0) {
                m.erase(cnt[arr[i]]);
            }
        }
        cnt[arr[i]]++;
        m[cnt[arr[i]]]++;

        if (diff.size() == 1) {
            mx = i + 1;
        }

        if (m.size() == 1 && m.begin()->first == 1) {
            mx = i + 1;
        } else if (m.size() == 2) {
            auto ss = ++m.begin();
            if (ss->first - m.begin()->first == 1 && ss->second == 1) {
                mx = i + 1;
            }
            if (m.begin()->first == 1 && m.begin()->second == 1) {
                mx = i + 1;
            }
        }
    }

    cout << mx << endl;

    return 0;
}