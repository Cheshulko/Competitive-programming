// https://www.codingame.com/ide/puzzle/hexagonal-maze
#include <algorithm>
#include <iostream>
#include <queue>
#include <string>
#include <vector>

using namespace std;

bool visit[25][25];
int deps[25][25];
pair<int, int> parent[25][25];
string grid[25];
int mx = 26 * 26 + 1;

queue<pair<int, int> > q;

/*
2 1
SE

2 2
##
SE

5 6
#####
#S#E#
#_#_#
#_#_#
#___#
#####
*/

int main() {
    int w;
    int h;
    cin >> w >> h;
    cin.ignore();
    for (int i = 0; i < h; i++) {
        getline(cin, grid[i]);
    }
    int is, js;
    int ie, je;
    for (int i = 0; i < h; ++i) {
        for (int j = 0; j < w; ++j) {
            visit[i][j] = false;
            deps[i][j] = mx;

            parent[i][j] = {-1, -1};

            if (grid[i][j] == 'S') {
                is = i;
                js = j;
            }
            if (grid[i][j] == 'E') {
                ie = i;
                je = j;
            }
        }
    }

    auto fix_i = [&](int i_to_fix) { return (i_to_fix + h) % h; };
    auto fix_j = [&](int j_to_fix) { return (j_to_fix + w) % w; };
    auto can_go = [&](int i_to_go, int j_to_go) {
        return grid[i_to_go][j_to_go] != '#';
    };

    pair<int, int> deltas[2][6] = {
        {{0, -1}, {0, +1}, {-1, -1}, {-1, 0}, {+1, -1}, {+1, 0}},
        {{0, -1}, {0, +1}, {-1, 0}, {-1, +1}, {+1, 0}, {+1, +1}},
    };

    int cur_i = is;
    int cur_j = js;
    deps[cur_i][cur_j] = 0;
    q.push({cur_i, cur_j});

    while (!q.empty() && !(cur_i == ie && cur_j == je)) {
        auto cur = q.front();
        q.pop();

        cur_i = cur.first;
        cur_j = cur.second;
        visit[cur_i][cur_j] = true;

        int odd = cur.first % 2;
        vector<pair<int, int> > directions;
        for (int try_to_go = 0; try_to_go < 6; ++try_to_go) {
            int to_go_i = fix_i(cur_i + deltas[odd][try_to_go].first);
            int to_go_j = fix_j(cur_j + deltas[odd][try_to_go].second);
            if (can_go(to_go_i, to_go_j)) {
                directions.push_back({to_go_i, to_go_j});
            }
        }

        for (int d = 0; d < directions.size(); ++d) {
            auto dir = directions[d];

            if (!visit[dir.first][dir.second])
                q.push(dir);
            deps[dir.first][dir.second] = deps[cur_i][cur_j] + 1;
            if (parent[dir.first][dir.second].first == -1)
                parent[dir.first][dir.second] = cur;
        }
    }

    cur_i = ie;
    cur_j = je;

    while (!(cur_i == is && cur_j == js)) {
        if (grid[cur_i][cur_j] != 'S' && grid[cur_i][cur_j] != 'E') {
            grid[cur_i][cur_j] = '.';
        };
        auto par = parent[cur_i][cur_j];
        cur_i = par.first;
        cur_j = par.second;
    }

    for (int i = 0; i < h; i++) {
        cout << grid[i] << endl;
    }
}