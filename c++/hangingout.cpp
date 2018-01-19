#include <iostream>
#include <string>

int main() {
    std::ios_base::sync_with_stdio(false);

    int l, x, k = 0, j;
    std::string p;

    std::cin >> l >> x;
    while(x--) {
        std::cin >> p >> j;
        if(p == "enter") {
            if(l - j >= 0)
                l -= j;           
            else
                k++;
        }
        else
            l += j;
    }
    std::cout << k << std::endl;
}
