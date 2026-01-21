// https://leetcode.com/problems/construct-the-minimum-bitwise-array-ii

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* minBitwiseArray(int* nums, int numsSize, int* returnSize) {
    *returnSize = numsSize;

    int* ans = (int*)malloc(numsSize * sizeof(int));
    for (int i = 0; i < numsSize; ++i) {
        if (!(nums[i] & 1)) {
            ans[i] = -1;

            continue;
        }

        for (long p = 1; (nums[i] >= p) && (nums[i] & p); p <<= 1) {
            ans[i] = nums[i] ^ p;
        }
    }

    return ans;
}