    # define _CRT_SECURE_NO_WARNINGS
    # include <unordered_map> 
    # include <functional>
    # include <algorithm>
    # include <iostream>
    # include <iomanip>
    # include <fstream>
    # include <sstream>
    # include <vector>
    # include <string>
    # include <bitset>
    # include <cmath>
    # include <queue>
    # include <stack>
    # include <ctime>
    # include <set>
    # include <map>
    # include <string.h>
    # include <limits.h>
    # include <stdlib.h>
    # include <stdio.h>
     
    using namespace std;
    typedef long long                  ll;
    typedef pair<long long, long long> pll;
    typedef pair<int, int>             pii;
    typedef pair<double, int>          pdi;
    typedef pair<double, double>       pdd;
    typedef pair<string, string>       pss;
    typedef unsigned long long         ull;
     
    int t;
    int xg[2], yg[2];
    char c[2];
    int used[10][10];
     
    int dx[] = {  2,  1, -1, -2, -2, -1, 1, 2 };
    int dy[] = { -1, -2, -2, -1,  1,  2, 2, 1 };
     
    int ans;
    queue<pair<pair<int, int>, int>> q;
     
    bool ok(int x, int y){
    	return (x <= 8 && x >= 1 && y <= 8 && y >= 1) && !used[x][y];
    }
     
    void bfs(){
    	while (!q.empty()){
    		pair<pair<int, int>, int> now = q.front();
    		q.pop();
    		int x = now.first.first;
    		int y = now.first.second;
    		int cnt = now.second;
    		used[x][y] = 1;
     
    		if (x == xg[1] && y == yg[1]){
    			ans = cnt;
    			return;
    		}
     
    		for (int i = 0; i < 8; ++i){
    			int tox = x + dx[i];
    			int toy = y + dy[i];
    			
    			if (ok(tox, toy)){
    				q.push({ { tox, toy }, cnt + 1 });
    			}
    		}
    	}    	
    }
     
    int main(){    
    	scanf("%d", &t);
    	for (int _t = 0; _t < t; ++_t){
    		scanf("\n%c%d %c%d", c, xg, c + 1, xg + 1);
    		yg[0] = c[0] - 'a' + 1;
    		yg[1] = c[1] - 'a' + 1;
    		memset(used, 0, sizeof used);
    		ans = 0;
    		while (!q.empty()) q.pop();
    		q.push({ { xg[0], yg[0] }, 0 });
    		bfs();
    		printf("%d\n", ans);
    	}
    	return 0;
    } 
