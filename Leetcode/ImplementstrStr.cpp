class Solution {
public:
    int strStr(string haystack, string needle) {
        if(needle == "") return 0;
        
        string s = needle + '#' + haystack;
        
        int n = (int) s.length();
	    vector<int> z (n);
	    for (int i=1, l=0, r=0; i<n; ++i) {
		if (i <= r)
			z[i] = min (r-i+1, z[i-l]);
		while (i+z[i] < n && s[z[i]] == s[i+z[i]])
			++z[i];
		if (i+z[i]-1 > r)
			l = i,  r = i+z[i]-1;
	    }
	    
	    int nl = needle.length();
	    for(int i = nl; i < s.length(); ++i){
	        if(z[i] == nl) return i - nl - 1; 
	    }
	    return -1;
        
    }
};
