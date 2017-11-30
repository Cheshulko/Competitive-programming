// https://www.codingame.com/training/medium/stock-exchange-losses

#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
 
int mx = (1 << 31);
int ans = 0;
int main()
{
    int n;
    cin >> n; cin.ignore();
    for (int i = 0; i < n; i++) {
        int v;
        cin >> v; cin.ignore();
        if(v < mx) ans = min(ans, v - mx);
        else mx = v;
    }

    cout << ans << endl;
}
