// https://www.codingame.com/ide/puzzle/the-gift
#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int l[2000];
int r[2000];
int n;
int b;

int main() {
    cin >> n;
    cin >> b;
    for (int i = 0; i < n; ++i)
        cin >> l[i];
    sort(l, l + n);

    int s = 0;
    for (int i = 0; i < n - 1; ++i) {
        int now_pls_cnt = (n - i);
        int avg = b / now_pls_cnt;
        int left = b % now_pls_cnt;

        int pay = min(l[i], avg);
        s += pay;
        r[i] = pay;
        b -= pay;
    }

    if (b <= l[n - 1]) {
        r[n - 1] = b;
        for (int i = 0; i < n; ++i) {
            cout << r[i] << endl;
        }
    } else {
        cout << "IMPOSSIBLE" << endl;
    }
}