// https://www.codingame.com/ide/puzzle/locked-in-gear
#include <algorithm>
#include <iostream>
#include <queue>
#include <string>
#include <vector>

using namespace std;

struct Gear {
    int mov = 0;
    bool visit = false;
    int x;
    int y;
    int r;
};

Gear gears[1001];

/*

2
1 1 1
1 3 1

3
1 1 1
1 3 1
3 3 1

*/

int main() {
    int n;
    cin >> n;
    cin.ignore();
    for (int i = 0; i < n; i++) {
        cin >> gears[i].x >> gears[i].y >> gears[i].r;
        cin.ignore();
    }

    auto is_connected = [&](const Gear& a, const Gear& b) {
        auto dx = a.x - b.x;
        auto dy = a.y - b.y;
        auto rs = a.r + b.r;

        return dx * dx + dy * dy == rs * rs;
    };

    queue<int> q;
    gears[0].mov = 1;
    q.push(0);
    while (!q.empty()) {
        auto index = q.front();
        q.pop();
        gears[index].visit = true;
        for (int i = 0; i < n; ++i) {
            if (i == index)
                continue;

            if (is_connected(gears[index], gears[i])) {
                auto& to_g = gears[i];
                auto to_g_mov = to_g.mov;
                auto g_mov = gears[index].mov;
                if (g_mov == to_g_mov) {
                    cout << "NOT MOVING" << endl;
                    return 0;
                }
                to_g.mov = -g_mov;
                if (!to_g.visit) {
                    q.push(i);
                }
            }
        }
    }
    cout << (gears[n - 1].mov == 1    ? "CW"
             : gears[n - 1].mov == -1 ? "CCW"
                                      : "NOT MOVING")
         << endl;
}