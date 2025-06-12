// https://leetcode.com/problems/maximum-difference-between-adjacent-elements-in-a-circular-array

int max(int a, int b) {
    return a > b ? a : b;
}

int maxAdjacentDistance(int* nums, int numsSize) {
    int dif = 0;
    for (int i = 0; i < numsSize; ++i) {
        dif = max(dif, abs(nums[i] - nums[(i + 1) % numsSize]));
    }

    return dif;
}