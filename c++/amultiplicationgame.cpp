#include <iostream>

int main() {
    unsigned long target;
    std::ios_base::sync_with_stdio(false);
    while(std::cin >> target) {
        unsigned long mem = 0, lel = 1;
        while(lel < target) {
            if(!mem)
                lel *= 9;
            else
                lel *= 2;
            mem = !mem;
        }
        std::cout << (mem ? "Stan wins." : "Ollie wins.") << std::endl;
    }
}
