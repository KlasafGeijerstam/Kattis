#include <iostream>

int main()
{
    std::ios_base::sync_with_stdio(false);
    int i, j;
    std::cin >> i >> j;
    std::cout << ((i*4 + j*3) & 1 ? "impossible" : "possible") << std::endl;
}
