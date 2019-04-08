#include <iostream>

using namespace std;

int pow(const int &a, int x)
{
    if (x)
    {
        int p = pow(a, x / 2);
        return p * p * (x % 2 ? a : 1);
    }
    else
    {
        return 1;
    }
}

int main()
{
    cout << pow(2, 10);
    return 0;
}

// Implement integer exponentiation. That is, implement the pow(x, y) function, where x and y are integers and returns x^y.

// Do this faster than the naive method of repeated multiplication.

// For example, pow(2, 10) should return 1024.