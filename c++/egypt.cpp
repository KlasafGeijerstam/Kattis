#include <iostream>
#include <algorithm>
#include <cmath>

int main() { 
    std::ios_base::sync_with_stdio(false);
    double x[3];
    double j, u, k, pi = 3.14159265359;
    while(true) {
        std::cin >> x[0] >> x[1] >> x[2];
        
        if(x[0] + x[1] + x[2] == 0)
            break;
        std::sort(x, x + 3);
        
        auto biglet = std::acos((x[0]*x[0] + x[1]*x[1] - x[2]*x[2])/(2*x[0]*x[1]));
        if(std::abs(biglet - (pi/2)) < 0.00000001)
            std::cout << "right" << std::endl;
        else
            std::cout << "wrong" << std::endl;
    }   
}
