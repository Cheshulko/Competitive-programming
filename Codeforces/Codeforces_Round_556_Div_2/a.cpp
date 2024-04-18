#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
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

int n, m, r;
int s, b;

int main(){

    cin >> n >> m >> r;
    int mn = 1111;
    int mx = -1111;
    for(int i = 0; i < n; ++i){
        cin >> s;
        mn = min(mn, s);
    }
    for(int i = 0; i < m; ++i){
        cin >> b;
        mx = max(mx, b);
    }

    if(mx <= mn){
        cout << r << endl;
        return 0;
    }

    int cnt = r / mn;
    int mon = cnt * mn;
    mon = r - mon;
    cout << (mx * cnt) + mon << endl;

    return 0;
}