// https://www.codingame.com/ide/puzzle/goro-want-chocolate
#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int dp[151][151];

int main() {
    int h;
    int w;
    cin >> h >> w;
    cin.ignore();

    for (int i = 1; i <= max(h, w); ++i) {
        for (int j = 1; j <= max(h, w); ++j) {
            dp[i][j] = 100500;
        }
    }

    for (int i = 1; i <= max(h, w); ++i) {
        dp[i][i] = 1;
    }
    for (int i = 1; i <= max(w, h); ++i) {
        dp[1][i] = dp[i][1] = i;
    }

    for (int i = 2; i <= h; ++i) {
        for (int j = 2; j <= w; ++j) {
            for (int k = 1; k <= min(i, j); ++k) {
                dp[i][j] = min(dp[i][j], dp[i - k][j] + dp[k][j - k] + 1);
                dp[i][j] = min(dp[i][j], dp[i][j - k] + dp[i - k][k] + 1);
            }
        }
    }

    cout << dp[h][w] << endl;
}