#include <iostream>
#include <stdio.h>
#include <vector>
#include <algorithm>
using namespace std;

int n;

long long ans = 0;
long long MOD = 998244353;

long long memo[1000005];
long long memoC[1000005];

int main(){
    memo[0] = 1;
    memo[1] = 1;
    memoC[0] = -1;
    for(int i = 1; i < 1000005; ++i){
        memo[i] = (memo[i - 1] * i) % MOD;
    }

    cin >> n;
    ans = memo[n];

    memoC[0] = 1;
    memoC[1] = n;
    for(int i = 2; i <= n; ++i){
        memoC[i] = (memoC[i - 1] * (n - i + 1)) % MOD;
    }

    for(int i = 0; i <= n - 1; ++i){
        long long ff = memo[i] - 1;
        long long c = memoC[n - i];
        long long add = ff * c % MOD;
        ans = (ans + add) % MOD;
    }
    cout << ans << endl;
    return 0;
}