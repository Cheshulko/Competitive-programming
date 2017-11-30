// https://www.codingame.com/training/medium/teads-sponsored-contest

#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

vector<vector<int>> g;
vector<int> d;
vector<int> used;

void dfs(int v){
    used[v] = 1;
    for(int i = 0; i < g[v].size(); ++i){
        int to = g[v][i];
        //cerr << "to " << to << endl;
        if(!used[to]){
            d[to] = d[v] + 1;
            dfs(to);
        }
    }
}
/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
int main()
{
    int N = 200001;
    int n; // the number of adjacency relations
    cin >> n; cin.ignore();
    g.resize(N);
    d.resize(N);
    
    for (int i = 0; i < n; i++) {
        int xi; // the ID of a person which is adjacent to yi
        int yi; // the ID of a person which is adjacent to xi
        cin >> xi >> yi; cin.ignore();
        cerr << xi << " " << yi << endl;
        g[xi].push_back(yi);
        g[yi].push_back(xi);        
    }

    used.assign(N, 0);
    dfs(0);
    int mxd = -1, v = -1;
    for(int i = 0; i < n; ++i){
        if(d[i] > mxd){
            mxd = d[i];
            v = i;
        }
    }
    d.assign(N, 0);
    used.assign(N, 0);
    dfs(v);
    mxd = -1, v = -1;
    for(int i = 0; i < n; ++i){
        if(d[i] > mxd){
            mxd = d[i];
            v = i;
        }
    }
    cout << mxd/2+mxd%2 << endl;
}
