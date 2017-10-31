Design a data structure that supports the following two operations:

void addWord(word)
bool search(word)

search(word) can search a literal word or a regular expression string containing only letters a-z or .. A . means it can represent any one letter.

For example:

addWord("bad")
addWord("dad")
addWord("mad")
search("pad") -> false
search("bad") -> true
search(".ad") -> true
search("b..") -> true

Note:
You may assume that all words are consist of lowercase letters a-z. 


class WordDictionary {
public:
    struct Node{
        int next[30];
        int leaf;
        Node(){
            for(int i = 0; i < 30; ++i) next[i] = -1;
            leaf = 0;
        }
    };
    
    vector<Node> trie;
    
    WordDictionary(){
        trie.resize(1);
    }

    // Adds a word into the data structure.
    void addWord(string word) {
        int len = word.length(), v = 0;
        for(int i = 0; i < len; ++i){
            if(trie[v].next[word[i] - 'a'] == -1){
                trie[v].next[word[i] - 'a'] = trie.size();
                trie.push_back(Node());
            }
            v = trie[v].next[word[i] - 'a'];
        }
        trie[v].leaf = 1;
    }

    // Returns if the word is in the data structure. A word could
    // contain the dot character '.' to represent any one letter.
    bool search(string word, int V = 0) {
        int len = word.length(), v = V;
        for(int i = 0; i < len; ++i){
            if(word[i] == '.'){
                bool have = false;
                for(int j = 0; j < 30; ++j){ 
                    if(trie[v].next[j] != -1){
                        string n = "";
                        n.insert(0, word, i + 1, word.size() - i - 1);
                        have |= search(n, trie[v].next[j]);
                    }   
                }
                
                return have;
            }
            else{
                if(trie[v].next[word[i] - 'a'] == -1) return false;
                v = trie[v].next[word[i] - 'a'];
            }
            
        }
        return trie[v].leaf;
    }
};

// Your WordDictionary object will be instantiated and called as such:
// WordDictionary wordDictionary;
// wordDictionary.addWord("word");
// wordDictionary.search("pattern");

