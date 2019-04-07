#include <iostream>
#include <string>
using namespace std;

int T;
int N;
string s;

int step = -1;
char dir[] = {'E', 'S'};

int main()
{
    cin >> T;
    for (int t = 0; t < T; ++t)
    {
        int x = 0, y = 0;
        string ans;
        cin >> N;
        cin >> s;
        int G = N - 1;
        N = 2 * (N - 1);

        if (s[0] == s[N - 1])
        {
            int L = 0;
            if (s[0] != dir[L])
                L = 1;

            for (int i = 0; i < N - 1; ++i)
            {
                if (s[i] == dir[1 - L] && s[i + 1] == dir[1 - L])
                {
                    step = i + 1;
                    break;
                }
            }

            int lx = 0, ly = 0;
            for (int i = 0; i < step; ++i)
            {
                if (s[i] == dir[0])
                    ++lx;
                else
                    ++ly;
            }

            L = 0;
            int firstDir = ly;
            if (s[0] != dir[L])
            {
                L = 1;
                firstDir = lx;
            }

            for (int i = 0; i < firstDir; ++i)
                ans.push_back(dir[1 - L]);

            for (int i = 0; i < G; ++i)
                ans.push_back(dir[L]);

            for (int i = 0; i < G - firstDir; ++i)
                ans.push_back(dir[1 - L]);
        }
        else
        {
            int L = 0;
            if (s[0] == dir[0])
                L = 1;

            for (int i = 0; i < G; ++i)
                ans.push_back(dir[L]);

            for (int i = 0; i < G; ++i)
                ans.push_back(dir[1 - L]);
        }

        cout << "Case #" << (t + 1) << ": " << ans << endl;
    }

    return 0;
}