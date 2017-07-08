#include <iostream>
#include <string>
#include <map>
#include <cmath>
using namespace std;

int main()
{
    int c, t;
    cin >> c >> t;

    for (size_t i = 1;; i++)
    {
        if (((int)ceil(i / (float)c)) == t) {
            cout << i << endl;
            return 0;
        }
    }
}
