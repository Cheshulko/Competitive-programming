// https://www.codingame.com/training/medium/the-last-crusade-episode-1

#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
 
int W; // number of columns.
int H; // number of rows.

pair<int, int> get(int x, int i, int j, int fromi, int fromj){
    
    cerr << "from : " << fromi << " " << fromj << " now : " << i << " " << j << endl;
    if(x == 0) {}
    if(x == 1) {return make_pair(i + 1, j);}
    if(x == 2) {if(fromj < j) return make_pair(i, j + 1); else return make_pair(i, j - 1);}
    if(x == 3) {return make_pair(i + 1, j);}
    if(x == 4) {if(fromi < i) return make_pair(i, j - 1); else return make_pair(i + 1, j);}
    if(x == 5) {if(fromi < i) return make_pair(i, j + 1); else return make_pair(i + 1, j);}
    if(x == 6) {
        if(fromj < j) return make_pair(i, j + 1); 
        else if(fromj > j) return make_pair(i, j - 1);
        else return make_pair(-1, -1);
    }
    if(x == 7) {return make_pair(i + 1, j);}
    if(x == 8) {return make_pair(i + 1, j);}
    if(x == 9) {return make_pair(i + 1, j);}
    if(x == 10) {if(fromi < i) return make_pair(i, j - 1); else return make_pair(-1, -1);}
    if(x == 11) {if(fromi < i) return make_pair(i, j + 1); else return make_pair(-1, -1);}
    if(x == 12) {return make_pair(i + 1, j);}
    if(x == 13) {return make_pair(i + 1, j);}
}

int mtx[22][22];
 
int main()
{
    
    cin >> W >> H; cin.ignore();
    for (int i = 0; i < H; i++) {
        for(int j = 0; j < W; ++j){
            int x;
            cin >> x;
            mtx[i][j] = x;   
            cerr << x << " ";
        }
        cerr << endl;
    }
    int EX; // the coordinate along the X axis of the exit (not useful for this first mission, but must be read).
    cin >> EX; cin.ignore();

    // game loop
    
    int fromi = -1, fromj;
    while (1) {
        int XI;
        int YI;
        string POS;
        cin >> XI >> YI >> POS; cin.ignore();
        if(fromi == -1) fromj = XI;
        
        cerr << mtx[YI][XI] << endl;
        auto res = get(mtx[YI][XI], YI, XI, fromi, fromj);
        cout << res.second << " " << res.first << endl;
        fromi = YI;
        fromj = XI;
     }
}
