#include <iostream>

int main() { 
    std::ios_base::sync_with_stdio(false);
    int x, y, z = 0, p;
    std::cin >> y >> x;
    while(y--) {
        std::cin >> p;
        x -= p;
        if(x >= 0)
            z++;
    }    
    std::cout << z << std::endl;
}
