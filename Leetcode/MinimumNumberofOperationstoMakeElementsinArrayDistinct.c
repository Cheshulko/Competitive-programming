// https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct

#define MAX 100

int max(int a, int b) {
    return a > b ? a : b;
}

int min(int a, int b) {
    return a > b ? b : a;
}

int minimumOperations(int* nums, int numsSize) {
    int pos_max_1[MAX + 1];
    int pos_max_2[MAX + 1];

    memset(pos_max_1, -1, (MAX + 1) * sizeof(int));
    memset(pos_max_2, -1, (MAX + 1) * sizeof(int));

    int mi = MAX;
    int ma = 1;

    for (int i = 0; i < numsSize; ++i) {
        if (pos_max_1[nums[i]] == -1) {
            pos_max_1[nums[i]] = pos_max_2[nums[i]] = i;
        } else {
            pos_max_2[nums[i]] = pos_max_1[nums[i]];
            pos_max_1[nums[i]] = i;
        }

        mi = min(mi, nums[i]);
        ma = max(ma, nums[i]);
    }

    int ans = 0;
    for (int cur = mi; cur <= ma; ++cur) {
        if (pos_max_1[cur] > pos_max_2[cur]) {
            ans = max(ans, 1 + pos_max_2[cur] / 3);
        }
    }

    return ans;
}
