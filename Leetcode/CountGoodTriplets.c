// https://leetcode.com/problems/count-good-triplets

int countGoodTriplets(int* arr, int arrSize, int a, int b, int c) {
    int ans = 0;

    for (int i = 0; i < arrSize; ++i) {
        for (int j = i + 1; j < arrSize; ++j) {
            for (int k = j + 1; k < arrSize; ++k) {
                int ok1 = abs(arr[i] - arr[j]) <= a;
                int ok2 = abs(arr[j] - arr[k]) <= b;
                int ok3 = abs(arr[i] - arr[k]) <= c;

                ans += ok1 & ok2 & ok3;
            }
        }
    }

    return ans;
}