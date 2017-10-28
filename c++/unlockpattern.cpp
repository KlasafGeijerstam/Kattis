#include <iostream>
#include <cmath>
#include <iomanip>

struct Point{
    int x, y;
};

double dist(Point p, Point y) {
    return std::sqrt(std::pow(p.x-y.x,2) + std::pow(p.y-y.y,2));
}

int main() {
    std::ios_base::sync_with_stdio(false);
    int m[3][3], x, y, z;
    for(int i = 0; i < 3; i++){
        std::cin >> x >> y >> z;
        m[0][i] = x;
        m[1][i] = y;
        m[2][i] = z;
    }
    int ind = 1;
    x = 0, y = 0;
    double dis = 0;
    while(1) {
        cont:
        if(ind == 10)
            break;
        for(int i = 0; i < 3; i++) {
            for(int h = 0; h < 3; h++) {
                if(m[i][h] == ind) {
                    if(ind != 1)
                        dis += dist({x, y}, {i, h});
                    x = i; y = h;
                    ind++;
                    goto cont;  
                }
            }  
        }         
    }
    std::cout << std::setprecision(10) << dis << std::endl;
} 
