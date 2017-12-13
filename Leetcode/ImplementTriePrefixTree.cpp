class TrieNode {
public:
	// Initialize your data structure here.
	int next[30];
	int leaf;
	TrieNode() {
		for (int i = 0; i < 30; ++i) next[i] = -1;
		leaf = 0;
	}
};

class Trie {
public:
	vector<TrieNode> trie;
	Trie() {		
		trie.resize(1);
	}

	// Inserts a word into the trie.
	void insert(string word) {
		int l = word.length(), v = 0;
		for(int i = 0; i < l; ++i){
			if (trie[v].next[word[i] - 'a'] == -1){
				trie[v].next[word[i] - 'a'] = trie.size();
				trie.push_back(TrieNode());

			}
			v = trie[v].next[word[i] - 'a'];
		}
		trie[v].leaf = true;
	}

	// Returns if the word is in the trie.
	bool search(string word) {
		int l = word.length(), v = 0;
		for (int i = 0; i < l; ++i){
			if (trie[v].next[word[i] - 'a'] == -1) return false;
			v = trie[v].next[word[i] - 'a'];
		}
		return trie[v].leaf;
	}

	// Returns if there is any word in the trie
	// that starts with the given prefix.
	bool startsWith(string prefix) {
		int l = prefix.length(), v = 0;
		for (int i = 0; i < l; ++i){
			if (trie[v].next[prefix[i] - 'a'] == -1) return false;
			v = trie[v].next[prefix[i] - 'a'];
		}
		return true;
	}
};



// Your Trie object will be instantiated and called as such:
// Trie trie;
// trie.insert("somestring");
// trie.search("key");
