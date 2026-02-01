// https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-i

int cmp(const void* a, const void* b) {
    int* ai = (int*)a;
    int* bi = (int*)b;

    return *ai - *bi;
}

int minimumCost(int* nums, int numsSize) {
    qsort(nums + 1, numsSize - 1, sizeof(*nums), cmp);

    return nums[0] + nums[1] + nums[2];
}