// https://www.codingame.com/training/medium/conway-sequence

#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

vector<long> v1, v2;
 
int main()
{
    int R;
    cin >> R; cin.ignore();
    int L;
    cin >> L; cin.ignore();

    v1.push_back(R);
    for(int l = 0; l < L - 1; ++l){
        for(int i = 0; i < v1.size(); ++i){
            int now = v1[i];
            int cnt = 0;
            while(i < v1.size() && now == v1[i]){
                ++cnt;
                ++i;
            }
            --i;
            v2.push_back(cnt);
            v2.push_back(now);
        }
        v1.assign(v2.begin(), v2.end());
        v2.clear();
    }

    for(int i = 0; i < v1.size() - 1; ++i){
        cout << v1[i] << " ";
    }
    cout << v1[v1.size() - 1] << endl;

}
