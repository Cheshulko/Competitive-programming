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
     
    void nextt(string& s){
    	int n = s.size();
    	int mid = n / 2;
    	if (n % 2){
    		if (s[mid] < '9'){
    			s[mid]++;
    			return;
    		}
    	}
    	else{
    		mid--;
    	}
    	for (int i = mid; i >= 0; --i){
    		if (s[i] < '9'){
    			s[i]++; s[n - i - 1]++;
    			for (int j = i + 1; j <= mid; ++j){
    				s[j] = s[n - j - 1] = '0';
    			}
    			return;
    		}
    	}
    	string ns = "1";
    	for (int i = 0; i < s.size() - 1; ++i) ns += "0";
    	ns += "1";
    	s = ns;
    }
     
    string getp(string s){
    	string r = s;
    	for (int i = 0; i < s.size() / 2; ++i){
    		r[i] = r[r.size() - i - 1] = s[i];
    	}
    	return r;
    }
     
    int main(){
    	int t;
    	string s;
    	cin >> t;
    	for (int _t = 0; _t < t; ++_t){
    		cin >> s;
    		string r = getp(s);
    		if (r > s) cout << r << endl;
    		else{
    			nextt(r); cout << r << endl;
    		}
    	}
    	return 0;
    } 
