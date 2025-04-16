// https://leetcode.com/problems/count-the-number-of-good-subarrays

#define MAX 100001

int comp(const void* a, const void* b) {
    int* ai = (int*)a;
    int* bi = (int*)b;

    return *ai - *bi;
}

size_t find(const int* arr, size_t size, int v) {
    size_t l = 0;
    size_t r = size;

    while (r - l > 1) {
        size_t m = (l + r) >> 1;

        if (arr[m] <= v) {
            l = m;
        } else {
            r = m;
        }
    }

    return l;
}

// Because we can. If you can't, buy more memory lol.
long long count[MAX];

long long countGood(int* nums, int numsSize, int k) {
    long long ans = 0;
    long long cur = 0;

    memset(count, 0, sizeof(long long) * MAX);

    // compress the array to avoid using the hash table
    int* ord = malloc(numsSize * sizeof(int));
    memcpy(ord, nums, numsSize * (sizeof(int) / sizeof(char)));
    qsort(ord, numsSize, sizeof(int), comp);

    long long l = 0;
    for (long long r = 0; r < numsSize; ++r) {
        size_t indR = find(ord, numsSize, nums[r]);
        long long v = count[indR]++;
        cur -= v * (v - 1) / 2;

        v = count[indR];
        cur += v * (v - 1) / 2;

        while (cur >= k && l < r) {
            size_t indL = find(ord, numsSize, nums[l]);
            long long v = count[indL];
            long long ncur = cur;
            ncur -= v * (v - 1) / 2;

            long long nv = v - 1;
            ncur += nv * (nv - 1) / 2;

            if (ncur >= k) {
                l += 1;
                cur = ncur;
                count[indL] = nv;
            } else {
                break;
            }
        }

        if (cur >= k) {
            ans += l + 1;
        }
    }

    return ans;
}