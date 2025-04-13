// https://leetcode.com/problems/count-good-numbers

#define MOD 1000000000 + 7

long long binPow(long long a, long long b) {
    long long ans = 1;
    while (b > 0) {
        if (b & 1) {
            ans *= a;
            ans %= MOD;
        }
        a *= a;
        a %= MOD;
        b >>= 1;
    }

    return ans;
}

int countGoodNumbers(long long n) {
    long long odd = n / 2;
    long long even = (n + 1) / 2;

    long long ans = 1;
    ans *= binPow(5, even);
    ans %= MOD;
    ans *= binPow(4, odd);
    ans %= MOD;

    return ans;
}