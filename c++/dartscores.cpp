#include <iostream>
#include <cmath>
#include <cstdio>

int main() {
    std::ios_base::sync_with_stdio(false);
    int cases, sc, x, y;
    std::cin >> cases;
    while(cases--) {
        std::cin >> sc;
        int score = 0;
        while(sc--) {
            std::cin >> x >> y;
            double r = sqrt(x*x + y*y);
            if(r > 200)
                continue;
            else if(r <= 20)
                score += 10;
            else if(r <= 40)
                score += 9;
            else if(r <= 60)
                score += 8;
            else if(r <= 80)
                score += 7;
            else if(r <= 100)
                score += 6;
            else if(r <= 120)
                score += 5;
            else if(r <= 140)
                score += 4;
            else if(r <= 160)
                score += 3;
            else if(r <= 180)
                score += 2;
            else if(r <= 200)
                score += 1;
        }
        printf("%d\n", score);
    }
}
