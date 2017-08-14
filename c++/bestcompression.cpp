#include <iostream>

int main()
{
    std::ios_base::sync_with_stdio(false);
    unsigned long long n,b;
    std::cin >> n >> b;
    std::cout << ((1ull << (b+1)) - 1 >= n ? "yes" : "no") << std::endl;
}
