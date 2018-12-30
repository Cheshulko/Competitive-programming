#include <iostream>
#include <stdio.h>
#include <set>
#include <map>
using namespace std;

int n;
int x[1111], y[1111];
int a[1111], b[1111];

map<pair<int, int> , int> sum;

int main(){
    cin >> n;
    for(int i = 0; i < n; ++i){
        cin >> x[i] >> y[i];
    }
    for(int i = 0; i < n; ++i){
        cin >> a[i] >> b[i];
    }
    for(int i = 0; i < n; ++i){
        for(int j = 0; j < n; ++j){
            pair<int, int> p = make_pair(x[i] + a[j], y[i] + b[j]);
            sum[p]++;
        }
    }
    for(auto ss : sum){
        if(ss.second == n){
            cout << ss.first.first << " " << ss.first.second << endl;
            return 0;
        }
    }

    return 0;
}