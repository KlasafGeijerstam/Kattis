#include <iostream>
#include <vector>
#include <algorithm>
#include <cinttypes>
using namespace std;

int main()
{
    int n;
    cin >> n;
    int d;
    uint8_t b[256];
    for(auto i = 0; i < 256; i++)
        b[(i^(i << 1))&255] = i;
    while(n--) {
        cin >> d;
        cout << (int)b[d] << " ";
    }
    cout << endl;
}
