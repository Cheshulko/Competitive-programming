#include <iostream>
#include <vector>
using namespace std;

int bs(const vector<int> &arr, int l, int r, int v) // [l; r]
{
    if (r < l)
        return 1487;
    if (r == l)
        return l;
    int m = (l + r) / 2;
    if (arr[l] <= arr[m]) // norm
    // [10, 13, (18), 25, 2, 8]
    {
        if (arr[l] <= v && v <= arr[m])
        {
            return bs(arr, l, m, v);
        }
        else
        {
            return bs(arr, m + 1, r, v);
        }
    }
    // [20, 25, 2, (8), 10, 12, 15]
    else
    {
        if (arr[m] <= v && v <= arr[r])
        {
            return bs(arr, m, r, v);
        }
        else
        {
            return bs(arr, l, m - 1, v);
        }
    }
}

int main()
{
    vector<int> arr = {13, 18, 25, 2, 8, 10};

    cout << bs(arr, 0, arr.size() - 1, 8) << endl;
    return 0;
}

// An sorted array of integers was rotated an unknown number of times.

// Given such an array, find the index of the element in the array in faster than linear time. If the element doesn't exist in the array, return null.

// For example, given the array [13, 18, 25, 2, 8, 10] and the element 8, return 4 (the index of 8 in the array).

// You can assume all the integers in the array are unique.