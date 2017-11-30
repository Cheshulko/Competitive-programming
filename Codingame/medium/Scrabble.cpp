// https://www.codingame.com/training/medium/scrabble

#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

int w[256];
  
vector<string> words;
  
int main()
{
    
    w['q'] = w['z'] = 9;
    w['j'] = w['x'] = 7;
    w['k'] = 4;
    w['f'] = w['h'] = w['v'] = w['w'] = w['y'] = 3;
    w['b'] = w['c'] = w['m'] = w['p'] = 2;
    w['d'] = w['g'] = 1;
    
    int N;
    string res;
    int mx = -1;
    cin >> N; cin.ignore();
    for (int i = 0; i < N; i++) {
        string W;
        getline(cin, W);
        words.push_back(W);        
    }
    string LETTERS;
    vector<int> lattest(256);
    getline(cin, LETTERS);
    for(int i = 0; i < LETTERS.size(); ++i){
        lattest[LETTERS[i] - 'a']++;
    }
    vector<int> now(256);    
    for(int i = 0; i < N; ++i){
        int r = 0;
        bool can = true;
        now.assign(256, 0);
        for(int j = 0; j < words[i].length(); ++j){
            int t = words[i][j] - 'a'; 
            now[t]++;      
            if(now[t] > lattest[t]){
                can = false;
                break;
            }
            r += (w[words[i][j]] + 1);   
        }

        if(can && mx < r){
            mx = r;
            res = words[i];
        }    
    }
    cout << res << endl;
}
