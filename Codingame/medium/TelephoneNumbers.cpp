// https://www.codingame.com/training/medium/telephone-numbers

#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

struct item {
  int next[10];
  int leaf;
  item(){
    for(size_t i = 0; i < 10; i++) next[i] = -1;
    leaf = 0;
  }
};

vector<item> trie[10];

void add_string (string &s){
  int i;
  int v = 0;
  int st = s[0] - '0';
  item temp;
  for (i = 0; i < s.length(); ++i){
    char c = s[i] - '0';
    if (trie[st][v].next[c] == -1){
      trie[st][v].next[c] = trie[st].size();      
      trie[st].push_back(temp);
    }
    v = trie[st][v].next[c];
  }
  trie[st][v].leaf = true;
}


/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
int main()
{
    
    int N;
    cin >> N; cin.ignore();
    for (int i = 0; i < 10; ++i) {
        trie[i].clear(); trie[i].resize(1);
        
    }
    for (int i = 0; i < N; i++) {
        string telephone;
        cin >> telephone; cin.ignore();
        add_string(telephone);
    }

    int res = 0;
    for(int i = 0; i < 10; ++i){
        res += trie[i].size() - 1;
    }

    cout << res << endl;
}
