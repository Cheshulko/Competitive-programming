#include <algorithm>
#include <cmath>
#include <iostream>
#include <queue>
#include <set>
#include <stack>
#include <unordered_set>
#include <vector>

using namespace std;

const int inf = 1e9;

struct Node {
    Node *l = 0, *r = 0;
    int lo, hi, mset = inf, madd = 0, val = 0;
    Node(int lo, int hi) : lo(lo), hi(hi) {}
    Node(vector<int>& v, int lo, int hi) : lo(lo), hi(hi) {
        if (lo + 1 < hi) {
            int mid = lo + (hi - lo) / 2;
            l = new Node(v, lo, mid);
            r = new Node(v, mid, hi);
            val = l->val + r->val;
        } else
            val = v[lo];
    }
    int query(int L, int R) {
        if (R <= lo || hi <= L)
            return 0;
        if (L <= lo && hi <= R)
            return val;
        push();
        return l->query(L, R) + r->query(L, R);
    }
    void set(int L, int R, int x) {
        if (R <= lo || hi <= L)
            return;
        if (L <= lo && hi <= R)
            mset = val = x, madd = 0;
        else {
            push(), l->set(L, R, x), r->set(L, R, x);
            val = l->val + r->val;
        }
    }
    void add(int L, int R, int x) {
        if (R <= lo || hi <= L)
            return;
        if (L <= lo && hi <= R) {
            if (mset != inf)
                mset += x;
            else
                madd += x;
            val += x;
        } else {
            push(), l->add(L, R, x), r->add(L, R, x);
            val = l->val + r->val;
        }
    }
    void push() {
        if (!l) {
            int mid = lo + (hi - lo) / 2;
            l = new Node(lo, mid);
            r = new Node(mid, hi);
        }
        if (mset != inf)
            l->set(lo, hi, mset), r->set(lo, hi, mset), mset = inf;
        else if (madd)
            l->add(lo, hi, madd), r->add(lo, hi, madd), madd = 0;
    }
};

int main() {
    cin.tie(0)->sync_with_stdio(0);
    cin.exceptions(cin.failbit);

    int t;
    cin >> t;

    while (t--) {
        int m, x;
        cin >> m >> x;

        vector<pair<int, int>> c;
        for (int i = 0; i < m; ++i) {
            int cc;
            cin >> cc;
            c.push_back({cc, i});
        }

        sort(c.begin(), c.end());

        int ans = 0;
        vector<int> v(m, x);
        auto st = Node(v, 0, m);

        for (int i = 0; i < m; ++i) {
            auto cc = c[i].first;
            auto j = c[i].second;

            if (j == 0) {
                continue;
            }

            auto can = st.query(0, j);
            if (can >= cc) {
                auto l = 0;
                auto r = j;

                while (r - l > 1) {
                    auto mid = (l + r) / 2;
                    auto try_can = st.query(mid, j);

                    if (try_can >= cc) {
                        can = try_can;
                        l = mid;
                    } else {
                        r = mid;
                    }
                }

                ans += 1;
                st.set(l, j, 0);
                st.add(l, l + 1, can - cc);
            }
        }

        cout << ans << endl;
    }
}