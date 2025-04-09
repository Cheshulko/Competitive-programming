// https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k

int comp(const void* a, const void* b) {
    int* ai = (int*)a;
    int* bi = (int*)b;

    return *ai - *bi;
}

int minOperations(int* nums, int numsSize, int k) {
    qsort(nums, numsSize, sizeof(*nums), comp);

    if (k > nums[0]) {
        return -1;
    }

    int ans = 0;
    for (int i = numsSize - 1; i > 0; --i) {
        ans += nums[i] > nums[i - 1];
    }

    return ans + (k < nums[0]);
}