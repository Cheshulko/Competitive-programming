// https://leetcode.com/problems/count-symmetric-integers

int countSymmetricIntegers(int low, int high) {
    int ans = 0;
    for (int i = low; i <= high; ++i) {
        int cnt = 0;
        int num = i;
        while (num)
            cnt += 1, num /= 10;

        if (cnt % 2 == 0) {
            int l = 0;
            int r = 0;

            num = i;
            for (int j = 0; j < cnt / 2; ++j)
                l += num % 10, num /= 10;
            for (int j = 0; j < cnt / 2; ++j)
                r += num % 10, num /= 10;

            ans += l == r;
        }
    }

    return ans;
}