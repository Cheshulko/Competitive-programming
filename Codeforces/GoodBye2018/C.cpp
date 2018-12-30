#include <iostream>
#include <stdio.h>
#include <vector>
#include <algorithm>
using namespace std;

long long n;

vector<long long> ans;

int main(){
    cin >> n;

    for(long long i = 1; i * i <= n; ++i){
        if(n % i == 0){
            long long r = (2 + i * (n / i - 1)) * (n / i) / 2;
            ans.push_back(r);
            long long rev = n / i;
            
            if(rev != i){
                long long r2 = (2 + rev * (n / rev - 1)) * (n / rev) / 2;
                ans.push_back(r2);
            }
            
        }
    }
    sort(ans.begin(), ans.end());
    for(int i = 0; i < ans.size(); ++i){
        cout << ans[i] << " ";
    }
    cout << endl;

    return 0;
}