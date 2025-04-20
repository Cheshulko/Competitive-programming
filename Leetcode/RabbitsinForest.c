// https://leetcode.com/problems/rabbits-in-forest

#define MAX 1001

int cnt[MAX];

int numRabbits(int* answers, int answersSize) {
    memset(cnt, 0, MAX * sizeof(int));

    int ans = 0;
    for (int i = 0; i < answersSize; ++i) {
        if (cnt[answers[i]] == 0) {
            ans += answers[i] + 1;
            cnt[answers[i]] = answers[i];
        } else {
            --cnt[answers[i]];
        }
    }

    return ans;
}