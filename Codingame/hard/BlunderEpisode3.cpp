// https://www.codingame.com/ide/puzzle/blunder-episode-3

#include <algorithm>
#include <cmath>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

struct Item {
    double input;
    double res;

    double logn;
    double n2;
    double n3;

    double calc(int t) {
        switch (t) {
            case 1:
                return c1();
            case 2:
                return c2();
            case 3:
                return c3();
            case 4:
                return c4();
            case 5:
                return c5();
            case 6:
                return c6();
            case 7:
                return c7();
            case 8:
                return c8();
        }
    }

    double c1() { return res / 1; }
    double c2() { return res / logn; }
    double c3() { return res / input; }
    double c4() { return res / (input * logn); }
    double c5() { return res / n2; }
    double c6() { return res / (n2 * logn); }
    double c7() { return res / n3; }
    double c8() { return res / (pow(2, input)); }
};

string types[8] = {
    "O(1)",   "O(log n)",     "O(n)",   "O(n log n)",
    "O(n^2)", "O(n^2 log n)", "O(n^3)", "O(2^n)",
};

vector<Item> nums;

int main() {
    int n;
    cin >> n;
    nums.resize(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i].input >> nums[i].res;
        nums[i].logn = log(1.0 * nums[i].input);
        nums[i].n2 = nums[i].input * nums[i].input;
        nums[i].n3 = nums[i].n2 * nums[i].input;
    }

    double diff = 1000000000000;
    int index = -1;
    double g[1001];

    for (int t = 1; t <= 8; ++t) {
        double sum = 0;
        for (int i = 0; i < n; ++i) {
            g[i] = nums[i].calc(t);
        }
        double mn = *min_element(g, g + n);
        for (int i = 0; i < n; ++i) {
            g[i] /= mn;
            sum += g[i];
        }
        if (sum < diff) {
            diff = sum;
            index = t;
        }
    }

    cout << types[index - 1] << endl;
}