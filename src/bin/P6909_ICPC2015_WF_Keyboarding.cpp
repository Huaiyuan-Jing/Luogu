#include <bits/stdc++.h>
#define LL long long
#define mp(a, b) make_pair(a, b)
using namespace std;
// 生成下一个状态的函数
vector<vector<vector<pair<int, int>>>> generate_nxt(int r, int c, vector<string> &map)
{
    // 初始化下一个状态的数组
    vector<vector<vector<pair<int, int>>>> nxt(
        r, vector<vector<pair<int, int>>>(c, vector<pair<int, int>>(4, mp(-1, -1))));
    // 定义方向数组
    vector<int> d = {0, 1, 0, -1, 0};
    // 遍历地图的每一个位置
    for (int i = 0; i < r; ++i)
    {
        for (int j = 0; j < c; ++j)
        {
            // 遍历每个位置的四个方向
            for (int k = 0; k < 4; ++k)
            {
                int tmpx = i + d[k];
                int tmpy = j + d[k + 1];
                // 找到相同颜色的连续区域
                while (tmpx >= 0 && tmpx < r && tmpy >= 0 && tmpy < c && map[tmpx][tmpy] == map[i][j])
                {
                    tmpx += d[k];
                    tmpy += d[k + 1];
                }
                // 如果找到了不超出地图范围的位置，则更新下一个状态数组
                if (tmpx >= 0 && tmpx < r && tmpy >= 0 && tmpy < c)
                {
                    nxt[i][j][k] = mp(tmpx, tmpy);
                }
            }
        }
    }
    // 返回下一个状态数组
    return nxt;
}
int bfs(int r, int c, vector<string> &map, vector<vector<vector<pair<int, int>>>> &nxt, string target)
{
    struct node
    {
        int x, y, step, curr;
    };
    queue<node> q;
    vector<int> d = {0, 1, 0, -1, 0};
    q.push(node{0, 0, 0, -1});
    while (target[q.front().curr + 1] == map[0][0])
    {
        q.front().curr += 1;
    }
    vector<vector<vector<int>>> val(target.size() + 2, vector<vector<int>>(r, vector<int>(c, 0x3f3f3f3f)));
    while (!q.empty())
    {
        int x = q.front().x;
        int y = q.front().y;
        int step = q.front().step;
        int curr = q.front().curr;
        // cout << "x: " << x << " y: " << y << " step: " << step << " curr: " << curr << endl;
        q.pop();
        for (int k = 0; k < 4; ++k)
        {
            if (nxt[x][y][k] == mp(-1, -1))
            {
                continue;
            }
            int nx = nxt[x][y][k].first;
            int ny = nxt[x][y][k].second;
            int nstep = step + 1;
            int ncurr = curr;
            // cout << "nx: " << nx << " ny: " << ny << " nstep: " << nstep << " ncurr: " << ncurr << endl;
            if (val[ncurr + 1][nx][ny] <= nstep)
            {
                continue;
            }
            // cout << "passed" << endl;
            val[ncurr + 1][nx][ny] = nstep;
            while (map[nx][ny] == target[ncurr + 1])
            {
                ncurr += 1;
                if (ncurr == target.size() - 1)
                {
                    return nstep;
                }
            }
            q.push(node{nx, ny, nstep, ncurr});
        }
    }
    return -1;
}
int main()
{
    // ios::sync_with_stdio(0);
    // cin.tie(0), cout.tie(0);
    // cout << setprecision(20);
    int r, c;
    cin >> r >> c;
    vector<string> map(r);
    for (int i = 0; i < r; ++i)
    {
        cin >> map[i];
    }
    vector<vector<vector<pair<int, int>>>> nxt = generate_nxt(r, c, map);
    string target;
    cin >> target;
    if (target[target.size() - 1] != '*')
        target += '*';
    cout << bfs(r, c, map, nxt, target) + target.size() << endl;
    return 0;
}
