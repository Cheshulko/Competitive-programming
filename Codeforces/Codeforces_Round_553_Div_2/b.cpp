#include <cstring>
#include <iostream>
#include <vector>
using namespace std;

int n, m;
int arr[500][500];
int mask = 1;

int main() {
    cin >> n >> m;
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            cin >> arr[i][j];
        }
    }

    for (int k = 0; k <= 10; ++k) {
        mask = (1 << k);
        int inrows = 0;
        for (int i = 0; i < n; ++i) {
            bool row = 0;
            for (int j = 0; j < m; ++j) {
                bool have = ((mask & arr[i][j]) > 0);
                row |= have;
            }
            inrows += row;
        }

        if (inrows > 0) {
            vector<int> ans;
            bool needZero = true;
            if (inrows & 1) {
                needZero = false;
            }
            for (int i = 0; i < n; ++i) {
                int one = 0;
                int zero = 0;
                for (int j = 0; j < m; ++j) {
                    if (arr[i][j] & mask) {
                        one = j + 1;
                    } else {
                        zero = j + 1;
                    }
                }
                if (needZero && zero && one) {
                    ans.push_back(zero);
                    needZero = false;
                } else if (one) {
                    ans.push_back(one);
                } else {
                    ans.push_back(zero);
                }
            }
            if (!needZero) {
                cout << "TAK" << endl;
                for (int &x : ans) {
                    cout << x << " ";
                }
                cout << endl;
                return 0;
            }
        }
    }
    cout << "NIE" << endl;

    return 0;
}
