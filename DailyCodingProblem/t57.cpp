#include <iostream>
#include <vector>
using namespace std;

string s = "the quick brown fox jumps over the lazy dog";
int k = 10;
vector<const string> ans;

int main()
{
    int st = 0;
    int curLen = 0;
    string curs;
    for (int i = 0; i < s.length(); ++i)
    {
        st = i;
        while (s[i] != ' ' && i < s.length())
        {
            ++i;
        }
        if (i == s.length())
        {
            --i;
        }
        if (k - curLen >= i - st)
        {
            for (int j = st; j <= i; ++j)
            {
                curs.push_back(s[j]);
                curLen++;
            }
        }
        else
        {
            ans.push_back(curs);
            curs = "";
            curLen = 0;
            for (int j = st; j <= i; ++j)
            {
                curs.push_back(s[j]);
                curLen++;
            }
        }
    }
    if (!curs.empty())
    {
        ans.push_back(curs);
    }
    for (auto &&w : ans)
    {
        cout << w << endl;
    }
    return 0;
}

// Given a string s and an integer k, break up the string into multiple texts such that each text has a length of k or less. You must break it up so that words don't break across lines. If there's no way to break the text up, then return null.

// You can assume that there are no spaces at the ends of the string and that there is exactly one space between each word.

// For example, given the string "the quick brown fox jumps over the lazy dog" and k = 10, you should return: ["the quick", "brown fox", "jumps over", "the lazy", "dog"]. No string in the list has a length of more than 10.