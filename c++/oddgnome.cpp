#include <iostream>

int main() {
    std::ios_base::sync_with_stdio(false);
    int c, g, k, p, l;
    std::cin >> c;
    
    while(c--) {
        std::cin >> k;
        p = 0;
        while(k--) {
            std::cin >> g;
            if(p++ && g - l != 1) {
                std::cout << p << std::endl;
                while(k--) std::cin >> g;
                break;
            }
            l = g;            
        }
    }
}
