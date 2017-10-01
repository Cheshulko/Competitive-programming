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
     
    const int NMAX = 100002;
     
    int arr[NMAX];
    int pr[NMAX];
     
    vector<int> prms;
     
    int main(){
    	for (int i = 2; i < NMAX; ++i){
    		if (!pr[i]){
    			prms.push_back(i);
    			for (int j = i + i; j < NMAX; j += i){
    				pr[j] = 1;
    			}
    		}
    	}
     
    	int t, n, k;
    	cin >> t;
    	for (int i = 0; i < t; ++i){
    		cin >> n >> k;
     
    		vector<int>::iterator lastPr = upper_bound(prms.begin(), prms.end(), n);
    		lastPr--;
    		int posLast = (lastPr - prms.begin());
    		if (posLast + 1 < k){
    			cout << 0 << endl;;
    			continue;
    		}
     
    		if (k == 0){
    			cout << (1ll * n*(n - 1)) / 2 << endl;
    			continue;
    		}
     
    		int l = 0;
    		int r = k - 1;
    		long long ans = 0;
     
    		while (r <= posLast){
    			long long up = (n - prms[r] + 1ll) * (l - 1 >= 0 ? prms[l] - prms[l - 1] : 1);
    			ans += up;
    			++l;
    			++r;
    		}
    		cout << ans << endl;
    	}
    		return 0;
    } 
