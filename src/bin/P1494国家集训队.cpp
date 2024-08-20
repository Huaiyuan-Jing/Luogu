#include <bits/stdc++.h>
#define LL long long
#define mp(a, b) make_pair(a, b)
using namespace std;
struct query {
    int l, r, id, ans, block_pos;
};
vector<int> block_pos;
bool cmp_query(query a, query b)
{
    return block_pos[a.r] == block_pos[b.r] ? a.l < b.l : a.r < b.r;
}
bool cmp_id(query a, query b) {
    return a.id < b.id;
}
int main()
{
    ios::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    cout << setprecision(20);
    int n, m;
    cin >> n >> m;
    int block_size = n / sqrt(m * 2 / 3);
    block_pos = vector<int>(n);
    for (auto i = 0; i < n; ++i) {
        block_pos[i] = i / block_size;
    }
    auto socks = vector<int>(n);
    for (auto i = 0; i < n; ++i) {
        cin >> socks[i];
    }
    auto queries = vector<query>(m);
    for (auto i = 0; i < m; ++i) {
        cin >> queries[i].l >> queries[i].r;
        queries[i].l -= 1;
        queries[i].r -= 1;
        queries[i].id = i;
    }
    sort(queries.begin(), queries.end(), cmp_query);
    auto num_of_colors = vector<int>(n);
    long long s = 0;
    auto update = [&num_of_colors, &s](int color, int diff) {
        s -= num_of_colors[color] * 1LL * (num_of_colors[color] - 1) / 2;
        num_of_colors[color] += diff;
        s += num_of_colors[color] * 1LL * (num_of_colors[color] - 1) / 2;
    };
    for (auto i = 0, l = 1, r = 0; i < m; ++i)
    {
        for (; l < queries[i].l; ++l) {
            update(socks[l], -1);
        }
        for (; l > queries[i].l; --l) {
            update(socks[l - 1], 1);
        }
        for (; r < queries[i].r; ++r) {
            update(socks[r + 1], 1);
        }
        for (; r > queries[i].r; --r) {
            update(socks[r], -1);
        }
        queries[i].ans = s;
    }
    std::function<int(int, int)> gcd = [&](long long a, long long b) -> int {
        if (a < b)
            swap(a, b);
        return b == 0 ? a : gcd(b, a % b);
    };
    sort(queries.begin(), queries.end(), cmp_id);
    for (int i = 0; i < m; ++i) {
        long long up = queries[i].ans;
        long long down = (queries[i].r - queries[i].l + 1) * 1LL * (queries[i].r - queries[i].l) / 2;
        if (up == 0) {
            cout << "0/1" << '\n';
            continue;
        }
        long long tmp = gcd(up, down);
        up /= tmp;
        down /= tmp;
        cout << up << '/' << down << '\n';
    }
}
