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

int n, k;
int arr[100005];
int cnt[100005];

int fi[100000 + 5];
int la[100000 + 5];

int main() {
    cin >> n >> k;

    for (int i = 0; i < n; ++i) {
        fi[i] = 100000000;
        la[i] = -1;
    }

    int ans = n + 2 * n - 2;

    for (int i = 0; i < k; ++i) {
        cin >> arr[i];
        --arr[i];
        cnt[arr[i]]++;

        fi[arr[i]] = min(fi[arr[i]], i);
        la[arr[i]] = max(la[arr[i]], i);
    }

    for (int i = 0; i < n; ++i) {
        ans -= (cnt[i] > 0);
    }

    for (int i = 0; i < n - 1; ++i) {
        if (cnt[i] && cnt[i + 1]) {
            if (la[i] >= fi[i + 1]) --ans;
            if (la[i + 1] >= fi[i]) --ans;
        }
    }

    cout << ans << endl;

    return 0;
}