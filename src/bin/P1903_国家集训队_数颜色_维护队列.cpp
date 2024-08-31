#include <bits/stdc++.h>
#define LL long long
#define mp(a, b) make_pair(a, b)
using namespace std;
int SIZE;
struct Query
{
    int l, r, t, id;
    inline bool operator<(Query b)
    {
        if (l / SIZE != b.l / SIZE)
        {
            return l / SIZE > b.l / SIZE;
        }
        else if (r / SIZE != b.r / SIZE)
        {
            return r / SIZE > b.r / SIZE;
        }
        else
        {
            return t > b.t;
        }
    }
};
struct Replace
{
    int pos, to;
};
vector<int> process(vector<int> &color, vector<Query> &queries, vector<Replace> &replaces)
{
    sort(queries.begin(), queries.end());
    auto num_of_color = vector<int>(1500009, 0);
    auto ans = vector<int>(queries.size());
    for (int i = 0, l = 0, r = -1, t = -1, s = 0; i < queries.size(); ++i)
    {
        for (; r < queries[i].r; ++r)
        {
            if (++num_of_color[color[r + 1]] == 1)
                s++;
        }
        for (; r > queries[i].r; --r)
        {
            if (--num_of_color[color[r]] == 0)
                s--;
        }
        for (; l > queries[i].l; --l)
        {
            if (++num_of_color[color[l - 1]] == 1)
                s++;
        }
        for (; l < queries[i].l; ++l)
        {
            if (--num_of_color[color[l]] == 0)
                s--;
        }
        for (; t < queries[i].t; ++t)
        {
            swap(color[replaces[t + 1].pos], replaces[t + 1].to);
            if (replaces[t + 1].pos > r || replaces[t + 1].pos < l)
                continue;
            if (--num_of_color[replaces[t + 1].to] == 0)
                s--;
            if (++num_of_color[color[replaces[t + 1].pos]] == 1)
                s++;
        }
        for (; t > queries[i].t; --t)
        {
            swap(color[replaces[t].pos], replaces[t].to);
            if (replaces[t].pos > r || replaces[t].pos < l)
                continue;
            if (--num_of_color[replaces[t].to] == 0)
                s--;
            if (++num_of_color[color[replaces[t].pos]] == 1)
                s++;
        }
        ans[queries[i].id] = s;
    }
    return ans;
}
int main()
{
    ios::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    cout << setprecision(20);
    int n, m;
    cin >> n >> m;
    auto color = vector<int>(n);
    for (auto i = 0; i < n; ++i)
        cin >> color[i];
    SIZE = pow(n, 2.0 / 3.0);
    int t = 0;
    auto queries = vector<Query>();
    auto replaces = vector<Replace>();
    for (int i = 0, x, y, t = -1; i < m; ++i)
    {
        string op;
        cin >> op >> x >> y;
        if (op == "Q")
        {
            queries.emplace_back(Query{x - 1, y - 1, t, int(queries.size())});
            continue;
        }
        t++;
        replaces.emplace_back(Replace{x - 1, y});
    }
    auto ans = process(color, queries, replaces);
    for (int i = 0; i < ans.size(); ++i)
        cout << ans[i] << endl;
}
