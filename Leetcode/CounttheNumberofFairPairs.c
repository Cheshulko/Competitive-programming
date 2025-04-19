// https://leetcode.com/problems/count-the-number-of-fair-pairs
int comp(const void* a, const void* b) {
    int* ai = (int*)a;
    int* bi = (int*)b;

    return *ai - *bi;
}

int upper_bound(const int* arr, size_t size, int v) {
    int l = -1;
    int r = (int)size;

    while (r - l > 1) {
        int m = (l + r) >> 1;

        if (arr[m] <= v)
            l = m;
        else
            r = m;
    }

    return r;
}

long long countFairPairs(int* nums, size_t numsSize, int lower, int upper) {
    qsort(nums, numsSize, sizeof(int), comp);

    long long ans = 0;
    for (size_t i = 0; i < numsSize - 1; ++i) {
        const int num = nums[i];

        const size_t l = (size_t)upper_bound(nums + i + 1, numsSize - i - 1,
                                             lower - num - 1);
        const size_t r =
            (size_t)upper_bound(nums + i + 1, numsSize - i - 1, upper - num);

        ans += (long long)(r - l);
    }

    return ans;
}