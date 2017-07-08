#include <algorithm>
#include <iostream>
#include <iomanip>
using namespace std;

int main(){
    double m,c,a;
    double r,pi = 3.14159265358979323846;
    
    while(0 < 1){
        cin >> r >> m >> c;
        if(m == 0 && c == 0 && r == 0)
            return 0;
        a = r*2*r*2;
        cout << std::setprecision(6) << r*r*pi << " " << (c/(double)m)*a << endl;
    }
    
}
