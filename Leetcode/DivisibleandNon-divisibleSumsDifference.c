// https://leetcode.com/problems/divisible-and-non-divisible-sums-difference

int differenceOfSums(int n, int m) {
    int c1 = 0, c2 = 0;
    for (int i = 1; i <= n; ++i) {
        int k = (i % m == 0);

        c1 += (1 - k) * i;
        c2 += k * i;
    }

    return c1 - c2;
}