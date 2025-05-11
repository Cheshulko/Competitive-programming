// https://leetcode.com/problems/three-consecutive-odds

int threeConsecutiveOdds(int* arr, int arrSize) {
    for (int i = 0; i < arrSize - 2; ++i) {
        if (arr[i] & arr[i + 1] & arr[i + 2] & 1) {
            return 1;
        }
    }
    return 0;
}