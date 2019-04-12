#include <iostream>
#include <vector>
#include <map>
#include <queue>
using namespace std;


// Too slow
class Solution
{
  private:
    map<string, int> words;
    vector<vector<int>> graph;
    vector<string> tmp;

  public:
    int bfs(string fromString, string toString, int n)
    {
        int fromIndex = words[fromString] - 1;
        int toIndex = words[toString] - 1;

        queue<pair<int, int>> q;
        vector<bool> used;

        q.push({fromIndex, 0});
        used.assign(n, false);
        used[fromIndex] = true;

        while (!q.empty())
        {
            auto [index, depth] = q.front();

            q.pop();

            if (index == toIndex)
                return depth;

            for (int i = 0; i < graph[index].size(); ++i)
            {
                int tryTo = graph[index][i];
                if (!used[tryTo])
                {
                    used[tryTo] = true;
                    q.push({tryTo, depth + 1});
                }
            }
        }

        return 0;
    }

    int ladderLength(string beginWord, string endWord, vector<string> &wordList)
    {
        tmp = wordList;

        if (find(tmp.begin(), tmp.end(), beginWord) == tmp.end())
        {
            tmp.push_back(beginWord);
        }

        int cnt = 1;
        graph.resize(tmp.size());
        for (const string &word : tmp)
        {
            words[word] = cnt++;
        }
        for (const string &word : tmp)
        {
            int fromIndex = words[word];
            for (int i = 0; i < word.size(); ++i)
            {
                for (char c = 'a'; c <= 'z'; ++c)
                {
                    string s = word;
                    s[i] = c;
                    if (s == word)
                        continue;
                    int toIndex = words[s];
                    if (toIndex != 0)
                    {
                        graph[fromIndex - 1].push_back(toIndex - 1);
                    }
                }
            }
        }

        int ans = bfs(beginWord, endWord, tmp.size());
        return ans ? ans + 1 : ans;
    }
};

int main()
{
    string from = "hit";
    string to = "cog";
    vector<string> v = {"hot", "dot", "dog", "lot", "log", "cog"};

    Solution s;
    cout << s.ladderLength(from, to, v) << endl;
    return 0;
}