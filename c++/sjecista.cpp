#include <iostream>

int main() {
    int x;
    std::cin >> x;
    std::cout << ((x == 3) ? 0 : (x*(x - 1)*(x - 2)*(x - 3))/24) << std::endl;
}
