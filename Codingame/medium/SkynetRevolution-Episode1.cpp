// https://www.codingame.com/training/medium/skynet-revolution-episode-1

#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <queue>

using namespace std;

int mtx[501][501];
vector<int> ex;
int d[501];
int N; // the total number of nodes in the level, including the gateways


void bfs(int st){
    queue<int> q;
    q.push(st);
    for(int i = 0; i < N; ++i)
        d[i] = 77777777;
    d[st] = 0;
    while(!q.empty()){
        int cur = q.front();
        q.pop();
        for(int i = 0; i < N; ++i){
            if(mtx[cur][i] && d[i] > d[cur] + 1){
                d[i] = d[cur] + 1;
                q.push(i);            
            }            
        }
    }
}

void cut(){
    int mn  = 7777777;
    int indst = -1;
    int inden = -1;
    for(int i = 0; i < ex.size(); ++i){
        for(int j = 0; j < N; ++j){
            cerr << "have " << ex[i] << "->" << j << endl;
            if(mtx[ex[i]][j] && d[j] < mn){
                mn = d[j];
                indst = j;
                inden = ex[i];
            }
        }
    }
    
    cout << indst << " " << inden << endl;
    mtx[indst][inden] = mtx[inden][indst] = 0;
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
int main()
{
    // int N; // the total number of nodes in the level, including the gateways
    int L; // the number of links
    int E; // the number of exit gateways
    cin >> N >> L >> E; cin.ignore();
    for (int i = 0; i < L; i++) {
        int N1; // N1 and N2 defines a link between these nodes
        int N2;
        cin >> N1 >> N2; cin.ignore();
        // cout << N1 <<" " << N2 << endl;;
        mtx[N1][N2] = mtx[N2][N1] = 1;
    }
    for (int i = 0; i < E; i++) {
        int EI; // the index of a gateway node
        cin >> EI; cin.ignore();
        ex.push_back(EI);
    }

    // game loop
    while (1) {
        int SI; // The index of the node on which the Skynet agent is positioned this turn
        cin >> SI; cin.ignore();

        cerr << "Si " << SI << endl;
        bfs(SI);
        for(int i = 0; i < N; ++i)
        cerr << i << "- " << d[i] << endl;
        cut();
    }
}
