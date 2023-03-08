// https://www.codingame.com/ide/puzzle/blunder-episode-2
#include <algorithm>
#include <iostream>
#include <queue>
#include <string>
#include <vector>

using namespace std;

/*
6
0 5 1 2
1 10 0 3
2 20 3 4
3 5 5 E
4 10 E E
5 10 E E
*/

struct Room {
    int index;
    int money;
    int to_go[2];
    int result = 0;
    bool visit = false;
    int best = -1;
};

vector<Room> rooms;

void dfs(int index, int path_ans, int& ans) {
    Room& now = rooms[index];
    now.visit = true;
    now.best = path_ans;
    for (int i = 0; i < 2; ++i) {
        int index_to_go = now.to_go[i];
        if (index_to_go == -1) {
            ans = max(ans, path_ans);
        } else {
            Room room_to_go = rooms[index_to_go];
            if (!room_to_go.visit) {
                int to_go_ans = path_ans + room_to_go.money;
                if (to_go_ans > room_to_go.best) {
                    dfs(index_to_go, to_go_ans, ans);
                }
            }
        }
    }
    now.visit = false;
}

int main() {
    int n;
    cin >> n;
    rooms.resize(n);

    auto get_to_go = [&](int& result) {
        string g;
        cin >> g;
        if (g.size() == 1 && g[0] == 'E') {
            result = -1;
        } else {
            result = stoi(g);
        }
    };

    for (int i = 0; i < n; i++) {
        int ind;
        cin >> ind;
        rooms[ind].index = ind;
        cin >> rooms[ind].money;
        get_to_go(rooms[ind].to_go[0]);
        get_to_go(rooms[ind].to_go[1]);
    }

    int ans = -1;
    dfs(0, rooms[0].money, ans);
    cout << ans << endl;
}