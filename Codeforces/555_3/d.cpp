#include <iostream>

using namespace std;

long long n, N, k;
long long arr[100000 + 1];
long long up[100000 + 1];

int main() {
    cin >> n >> k;
    N = n;
    long long sum = 0;
    for (int i = 0; i < k; ++i) {
        arr[i] = i + 1;
        sum += arr[i];
    }
    if (sum > n) {
        cout << "NO" << endl;
        return 0;
    }
    n -= sum;
    sum = 0;
    for (int i = 0; i < k; ++i) {
        long long x = n / (k - i);
        if (i - 1 >= 0) {
            x = min(x, arr[i - 1] * 2 - arr[i]);
        }
        n -= x * (k - i);

        if (i - 1 >= 0) {
            up[i] += up[i - 1];
        }
        up[i] += x;

        arr[i] += up[i];
        sum += arr[i];
    }
    if (sum < N) {
        cout << "NO" << endl;
        return 0;
    }
    cout << "YES" << endl;
    for (int i = 0; i < k; ++i) {
        cout << arr[i] << " ";
    }
    cout << endl;

    return 0;
}