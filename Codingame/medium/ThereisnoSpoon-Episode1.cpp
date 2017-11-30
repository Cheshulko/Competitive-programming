// https://www.codingame.com/training/medium/there-is-no-spoon-episode-1

#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

/**
 * Don't let the machines win. You are humanity's last hope...
 **/
int main()
{
    
    int mtx[31][31];
    int width; // the number of cells on the X axis
    cin >> width; cin.ignore();
    int height; // the number of cells on the Y axis
    cin >> height; cin.ignore();
    for (int i = 0; i < height; i++) {
        string line; // width characters, each either 0 or .
        getline(cin, line);
        for(int j = 0; j < width; ++j){
            mtx[i][j] = (line[j] == '0');
        }
    }
    
    for(int i = 0; i < height; ++i){
        for(int j = 0; j < width; ++j){
            if(mtx[i][j]){
                int jj = j + 1; 
                int ii = i + 1;
                bool fw = 0;
                bool fh = 0;
                
                cout << j << " " << i << " ";
                while(jj < width && !fw){
                    if(mtx[i][jj]){
                        fw = 1;
                        cout << jj << " " << i << " ";
                    }
                    ++jj;
                }
                if(!fw) 
                    cout << -1 << " " << -1 << " ";
                 
                while(ii < height && !fh){
                   if(mtx[ii][j]){
                        fh = 1;
                        cout << j << " " << ii << endl;
                    }
                    ++ii;
                }
                if(!fh) 
                    cout << -1 << " " << -1 << endl;

            }
        }
    }
}
