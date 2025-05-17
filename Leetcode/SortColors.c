// https://leetcode.com/problems/sort-colors

void sortColors(int* nums, int numsSize) {
    int cnt[3] = {0};

    for (int i = 0; i < numsSize; ++i) {
        cnt[nums[i]] += 1;
    }

    for (int el = 0, i = 0; el < 3; ++el) {
        while (cnt[el]--) {
            nums[i++] = el;
        }
    }
}