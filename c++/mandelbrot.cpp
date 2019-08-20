#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <complex>
#include <iostream>

using namespace std;

int main(int argc, char** argv) {
    ios_base::sync_with_stdio(false);
    double x, y, r;
    int ca = 1;
    while(cin >> x >> y >> r) {
        complex<double> z(0);
        complex<double> c = x + y*1i;
        for(int iter = 0; abs(z) < 2 && iter < r; iter++) {
            z = (z*z) + c;
        }
        cout << "Case " << ca++ << ": ";
        if(abs(z) < 2) {
            cout << "IN";
        } else {
            cout << "OUT";
        }
        cout << endl;
    }
    return 0;
}