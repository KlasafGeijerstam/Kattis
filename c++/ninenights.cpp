#include <string>
#include <iostream>

int main() {
    std::ios_base::sync_with_stdio(false);
    std::string a[5];
    int i = 5, k = 0;

    while(i--)
        std::cin >> a[i];
    i = 0;
    for(int x = 0; x < 5; x++) {
        for(int y = 0; y < 5; y++) {
            if(a[x][y] == 'k') {
                k++;

                for(int xm = -1; xm < 2; xm += 2) {
                    for(int ym = -2; ym < 5; ym += 4) {
                        if(x + xm < 5 && x + xm >= 0 && y + ym < 5 && y + ym >= 0 && a[x + xm][y + ym] == 'k')
                            i++;
                        if(x + ym < 5 && x + ym >= 0 && y + xm < 5 && y + xm >= 0 && a[x + ym][y + xm] == 'k')
                            i++;
                    }
                }
            }
        }
    } 
    std::cout << (i || k != 9 ? "invalid" : "valid") << std::endl;
}
