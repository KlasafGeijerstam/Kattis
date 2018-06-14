#include <iostream>
#include <algorithm>
#include <iomanip>
#include <cmath>

using namespace std;

int p, a, b, c, d, n;

inline double va(int k)
{
    return p * (sin(a*k + b) + cos(c*k + d) + 2);
}

int main()
{
    ios_base::sync_with_stdio(false);
    cin >> p >> a >> b >> c >> d >> n;
    double max = 0, min = -1, val, nm = -1;
    for (int i = 1; i <= n; i++) {
        val = va(i);
        if (val > nm) {
            nm = val;
            min = val;
        }
        if (val < min)
            min = val;
        if (nm - min > max)
            max = nm - min;
    }
    cout << setprecision(12) << max << endl;
}
