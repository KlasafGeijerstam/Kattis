#include <iostream>
#include <vector>
#include <string>

int main() {
    std::ios_base::sync_with_stdio(false);
    int r,c;
    std::cin >> r >> c; 
    std::vector<std::string> m(r);
    int car[5]{0};
    for(int i = 0; i < r; i++)
        std::cin >> m[i];

    for(int i = 0; i < r - 1; i++) {
        for(int j = 0; j < c-1; j++) {
            int cars = 0;
            for(int k = 0; k < 2; k++)
                for(int l = 0; l < 2; l++)
                    if(m[i+k][j+l] == 'X')
                        cars++;
                    else if(m[i+k][j+l] == '#')
                        cars = -1337;
            if(cars >= 0)
                car[cars]++;
                         
        }
    }
    for(int i = 0; i < 5; i++)
        std::cout << car[i] << std::endl;
}
