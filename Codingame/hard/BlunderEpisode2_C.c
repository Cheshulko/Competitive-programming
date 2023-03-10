// https://www.codingame.com/ide/puzzle/blunder-episode-2

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
6
0 5 1 2
1 10 0 3
2 20 3 4
3 5 5 E
4 10 E E
5 10 E E

2
0 2 E 2
0 2 E 2
*/

struct Room {
    int index;
    int money;
    int to_go[2];
    int result;
    int visit;
    int best;
};

struct Room* rooms;

int mx(int* a, int* b) {
    return *a < *b ? *b : *a;
}

void dfs(int index, int path_ans, int* ans) {
    struct Room* now = &rooms[index];
    now->visit = 1;
    now->best = path_ans;
    for (int i = 0; i < 2; ++i) {
        int index_to_go = now->to_go[i];
        if (index_to_go == -1) {
            *ans = mx(ans, &path_ans);
        } else {
            struct Room* room_to_go = &rooms[index_to_go];
            if (!room_to_go->visit) {
                int to_go_ans = path_ans + room_to_go->money;
                if (to_go_ans > room_to_go->best) {
                    dfs(index_to_go, to_go_ans, ans);
                }
            }
        }
    }
    now->visit = 0;
}

int main() {
    int n;
    scanf("%d", &n);
    fgetc(stdin);
    rooms = (struct Room*)malloc(n * sizeof(struct Room));

    for (int i = 0; i < n; i++) {
        int ind;
        scanf("%d", &ind);
        rooms[ind].index = ind;
        scanf("%d", &rooms[ind].money);
        fgetc(stdin);

        char room[257];
        scanf("%[^\n]", room);
        fgetc(stdin);

        char* pch;

        pch = strtok(room, " ");
        for (int ex = 0; ex < 2; ++ex) {
            if (pch[0] == 'E') {
                rooms[ind].to_go[ex] = -1;
            } else {
                int v = atoi(pch);
                rooms[ind].to_go[ex] = v;
            }
            pch = strtok(NULL, " ");
        }
    }

    int ans = -1;
    dfs(0, rooms[0].money, &ans);
    free(rooms);
    printf("%d\n", ans);
}
