#include <iostream>

int main() {
    std::ios_base::sync_with_stdio(false);
    int n, s = 0, t = 0, c;
    std::cin >> n;
    while (n--) {
        std::cin >> c;
        t += c > 0 ? c : 0;
        s += c >= 0 ? 1 : 0;
    }
    std::cout << (t / (double)s) << std::endl;
}
