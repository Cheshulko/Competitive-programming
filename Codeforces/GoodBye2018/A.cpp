#include <iostream>
#include <stdio.h>
using namespace std;

int y, b, r;

int main(){

    cin >> y >> b >> r;
    int l = 0;
    while(1){
        if (l > y) break;
        if (l + 1 > b) break;
        if (l + 2 > r) break;
        ++l;
    }
    --l;
    cout << 3 * l + 3 << endl;

    return 0;
}