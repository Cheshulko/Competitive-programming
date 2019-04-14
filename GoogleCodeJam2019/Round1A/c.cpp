#include <iostream>
#include <vector>
using namespace std;

int T, n;
string s;

#define ALPHABET 27
struct item {
    int next[ALPHABET];
    int cnt;
    int leaf;
    item() {
        for (int i = 0; i < ALPHABET; i++) next[i] = -1;
        leaf = 0;
        cnt = 1;
    }
};

vector<item> trie;

void addString(const string &s) {
    int i, v = 0;
    item temp;
    for (i = s.length() - 1; i >= 0; --i) {
        char c = s[i] - 'A';
        int &to = trie[v].next[c];
        if (to == -1) {
            to = trie.size();
            trie.push_back(temp);
        } else {
            trie[to].cnt++;
        }
        v = trie[v].next[c];
    }
    trie[v].leaf = true;
}

int dfs(int v) {
    int cntV = trie[v].cnt;
    int have = 0;
    for (char c = 'A'; c <= 'Z' + 1; ++c) {
        int to = trie[v].next[c - 'A'];
        if (to != -1) {
            int cntTo = trie[to].cnt;
            if (cntTo > 1) {
                have += dfs(to);
            }
        }
    }
    cntV -= have;
    cntV -= cntV % 2;
    int ret = have;
    if (v != 0) {
        ret += min(2, cntV);
    }
    return ret;
}

item it;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> T;
    for (int t = 0; t < T; ++t) {
        cin >> n;
        trie.clear();
        trie.push_back(it);
        for (int i = 0; i < n; ++i) {
            cin >> s;
            addString(s);
        }
        cout << "Case #" << t + 1 << ": " << dfs(0) << endl;
    }

    return 0;
}