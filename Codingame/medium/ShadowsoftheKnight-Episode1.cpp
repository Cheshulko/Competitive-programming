// https://www.codingame.com/training/medium/shadows-of-the-knight-episode-1

#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

int x = 0, X;
int y = 0, Y;
int xcur = 0, ycur = 0;

void upd(char bombDir){
    if(bombDir == 'R'){
        cerr << "R" << endl;
                x = xcur;
                xcur = (X + x) / 2;
            }else if(bombDir == 'L'){
                cerr << "L" << endl;
                X = xcur;
                xcur = (X + x) / 2;
            }else if(bombDir == 'D'){
                cerr << "U" << endl;
                y = ycur;
                ycur = (Y + y) / 2;
            }else if(bombDir == 'U'){
                cerr << "D" << endl;
                cerr << y << " " << Y << endl;
                Y = ycur;
                ycur = (Y + y) / 2;
            }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
int main()
{
    int W; // width of the building.
    int H; // height of the building.
    cin >> W >> H; cin.ignore();
    int N; // maximum number of turns before game over.
    cin >> N; cin.ignore();
    int X0;
    int Y0;
    cin >> X0 >> Y0; cin.ignore();

    X = W, Y = H;

int st = 0;
xcur = X0;
ycur = Y0;
    // game loop
    while (1) {
        string bombDir; // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)
        cin >> bombDir; cin.ignore();

        // Write an action using cout. DON'T FORGET THE "<< endl"
        // To debug: cerr << "Debug messages..." << endl;
        
        for(int i = 0; i < bombDir.size(); ++i){
            upd(bombDir[i]);
        }
        cout << xcur << " " << ycur << endl;
        ++st;
    }
}
