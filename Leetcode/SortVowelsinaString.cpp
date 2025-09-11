// https://leetcode.com/problems/sort-vowels-in-a-string

class Solution {
   public:
    string sortVowels(string s) {
        static const vector<char> Vowels = {'a', 'e', 'i', 'o', 'u'};

        vector<size_t> indxs;
        for (auto i = 0; i < s.size(); ++i) {
            if (Vowels.cend() !=
                find(Vowels.cbegin(), Vowels.cend(), tolower(s[i]))) {
                indxs.push_back(i);
            }
        }

        vector<size_t> indxs2 = indxs;
        sort(indxs2.begin(), indxs2.end(),
             [&](const auto& a, const auto& b) { return s[a] < s[b]; });

        string ans = s;
        for (auto i = 0; i < indxs.size(); ++i) {
            ans[indxs[i]] = s[indxs2[i]];
        }

        return ans;
    }
};