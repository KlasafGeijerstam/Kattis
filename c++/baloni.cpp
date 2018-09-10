#include <iostream>

int mp[1000002];

int main() {
    std::ios_base::sync_with_stdio(false);
    int len, b;
    std::cin >> len;
    while(len--) {
        std::cin >> b;
        if(mp[b + 1])
            mp[b + 1]--;
        else
            mp[0]++;
        mp[b]++;
    }
    std::cout << mp[0] << std::endl;
}
