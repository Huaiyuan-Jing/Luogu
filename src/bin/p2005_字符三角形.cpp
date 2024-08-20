/********************************************************************************
 * @author: Huaiyuan Jing
 * @email: ls.hyjing@gmail.com
 * @version: 1.0
 * @description:
 ********************************************************************************/
#include <bits/stdc++.h>
#define LL long long
#define mp(a, b) make_pair(a, b)
using namespace std;
int main()
{
    ios::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    cout << setprecision(20);
    string s;
    cin >> s;
    for (int y = 0; y < 3; ++y)
    {
        for (int x = 0; x < 2 - y; ++x)
        {
            cout << " ";
        }
        for (int x = 0; x < 2 * y + 1; ++x)
        {
            cout << s;
        }
        for (int x = 0; x < 2 - y; ++x)
        {
            cout << " ";
        }
        cout << endl;
    }
}
