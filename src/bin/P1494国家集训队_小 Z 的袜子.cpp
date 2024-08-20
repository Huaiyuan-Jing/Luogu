#include <bits/stdc++.h>
#define LL long long
#define mp(a, b) make_pair(a, b)
using namespace std;
struct query {
    int l, r;
};
bool cmp_query(query a, query b) {
    return a.l != b.l ? a.l < b.l : a.r < b.r;
}
int main()
{
    ios::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    cout << setprecision(20);
    int n, m;
    cin >> n >> m;
    auto socks = vector<int>(n);
    for (auto i = 0; i < n; ++i) {
        cin >> socks[i];
    }
    auto queries = vector<query>(m);
    for (auto i = 0; i < m; ++i) {
        cin >> queries[i].l >> queries[i].r;
    }
    
}
