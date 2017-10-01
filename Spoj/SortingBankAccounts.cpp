    # define _CRT_SECURE_NO_WARNINGS
    # include <unordered_map> 
    # include <functional>
    # include <algorithm>
    # include <iostream>
    # include <iomanip>
    # include <fstream>
    # include <sstream>
    # include <vector>
    # include <string>
    # include <bitset>
    # include <cmath>
    # include <queue>
    # include <stack>
    # include <ctime>
    # include <set>
    # include <map>
    # include <string.h>
    # include <limits.h>
    # include <stdlib.h>
    # include <stdio.h>
     
    using namespace std;
    typedef long long                  ll;
    typedef pair<long long, long long> pll;
    typedef pair<int, int>             pii;
    typedef pair<double, int>          pdi;
    typedef pair<double, double>       pdd;
    typedef pair<string, string>       pss;
    typedef unsigned long long         ull;
     
    string s[10];
    map<string, int> mp;
     
    int main(){
    	int t;
    	cin >> t;
    	for (int i = 0; i < t; ++i){
    		mp.clear();
    		int n;
    		cin >> n;
    		for (int k = 0; k < n; ++k){
    			string l;
    			for (int j = 0; j < 6; ++j){
    				cin >> s[j];
    				l += s[j];
    				if (j < 5) l += " ";
    				//cout << "l = " << l << endl;
    			}
    			mp[l]++;
    		}
     
    		for (auto x : mp){
    			cout << x.first << " " << x.second << endl;;
    		}
    	}

    	return 0;
    } 
