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

int n;
char mtx[55][55];

void fill(int i, int j) {
    if (i + 1 < n && i + 2 < n && j - 1 >= 0 && j + 1 < n) {
        if (mtx[i][j] == '.' && 
            mtx[i + 1][j] == '.' && 
            mtx[i + 1][j - 1] == '.' && 
            mtx[i + 1][j + 1] == '.' && 
            mtx[i + 2][j] == '.') {
                
            mtx[i][j] = '#';
            mtx[i + 1][j] = '#';
            mtx[i + 1][j - 1] = '#';
            mtx[i + 1][j + 1] = '#';
            mtx[i + 2][j] = '#';

        }
    }
}

int main() {
    cin >> n;
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            cin >> mtx[i][j];
        }
    }
    for (int i = 0; i < n; ++i) {
        for(int j = 0; j < n; ++j){
            fill(i, j);
        }
    }
    for (int i = 0; i < n; ++i) {
        for(int j = 0; j < n; ++j){
            if(mtx[i][j] == '.'){
                cout << "NO" << endl;
                return 0;
            }
        }
    }
    cout << "YES" << endl;

    return 0;
}