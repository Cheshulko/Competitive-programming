#include <stdio.h>
#include <string>
#include <iostream>
#include <algorithm>
using namespace std;

int T;
string s;

int main()
{
    cin >> T;
    for (int t = 0; t < T; ++t)
    {
        cin >> s;
        string s2;
        for (int i = s.length() - 1; i >= 0; --i)
        {
            if (s[i] == '4')
            {
                s2.push_back('1');
                s[i] = '3';
            }
            else
            {
                s2.push_back('0');
            }
        }

        for (int i = s2.length() - 1; i >= 0; --i)
        {
            if (s2[i] == '0')
            {
                s2.pop_back();
            }
            else
            {
                break;
            }
        }

        reverse(s2.begin(), s2.end());

        if (s2.empty())
        {
            s2 = "0";
        }

        cout << "Case #" << (t + 1) << ": " << s << " " << s2 << endl;
    }

    return 0;
}