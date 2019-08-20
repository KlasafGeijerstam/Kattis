#include <iostream>
#include <cstring>
#include <algorithm>

using namespace std;
int n;
int amount[1000];
int mem[1000][1000];

int optimal(int flt, int jmp) {
    if (flt < 0 || flt >= n)
        return 100000000;
    if (flt == n - 1)
        return amount[flt];

    if (mem[flt][jmp] != -1)
        return mem[flt][jmp];

    return mem[flt][jmp] = amount[flt] + min(optimal(flt - jmp, jmp),optimal(flt + jmp + 1, jmp + 1));
}

int main() {
    cin >> n;
    for (int i = 0; i < n; ++i)
        cin >> amount[i];

    memset(mem, -1, sizeof mem);
    cout << optimal(1, 1) << endl;
    int p;
    cin >> p;
    return 0;
}
