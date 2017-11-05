#include <iostream>
#include <string>
#include <map>

int main() {
    std::ios_base::sync_with_stdio(false);
    std::map<std::string, int> m;
    std::string s;
    while (std::cin >> s) {
        m[s] = m[s] + 1;
        if (m[s] > 1) {
            std::cout << "no" << std::endl;
            return 0;
        }
    }   
    std::cout << "yes" << std::endl;
}
