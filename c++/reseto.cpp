#include <iostream>
#include <cmath>
#include <vector>

using namespace std;

int main()
{
    int n, k, a;
    cin >> n >> k;
    vector<int> p(n + 1);
    for (int i = 0; i <= n; i++)
        p[i] = i;

    while (true)
    {
        for (int i = 2; i <= n; i++)
            if (p[i] != -1)
            {
                int v = i;
                int index = 1;
                while (v*index <= n) {
                    if (p[v*index] != -1) {
                        p[v*index] = -1;
                        a = v*index;
                        k--;
                    }
                    index++;
                    if (!k)
                        goto end;
                }
            }
    }
end:
    cout << a << endl;
} 
