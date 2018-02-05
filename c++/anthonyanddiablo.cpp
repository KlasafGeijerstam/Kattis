#include <iostream>
#include <cmath>

int main() {
    double a, n, p = 3.14159265359;
    std::cin >> a >> n;
    std::cout << (std::sqrt((a/p))*2*p <= n ? "Diablo is happy!" : "Need more materials!") << std::endl;
}
