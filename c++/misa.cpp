#include <iostream>
#include <vector>
#include <algorithm>

int m = -30;
int r, s;

void check(char c[51][51]) {
    int shake = 0;
    
    for(int i = 0; i < r; i++) {
        for(int j = 0; j < s; j++) {
            if(c[i][j] != 'o')
                continue;
            for(int k = -1; k <= 1; k++) {
                for(int h = -1; h <= 1; h++){
                    if(i + k >= 0 && i + k < r && j+h >= 0 && j+h < s && !(k == 0 && h == 0) && c[i+k][j+h] == 'o') {
                        shake++;
                    }
                }
            }
        }         
    }
   m = std::max(shake, m); 
}


int main() {
    std::ios_base::sync_with_stdio(false);
    std::cin >> r >> s;
    char l[51][51];

    for(int i = 0; i < r; i++) {
        for(int j = 0; j < s; j++) {
            std::cin >> l[i][j];
        }         
    } 
    
    for(int i = 0; i < r; i++) {
        for(int j = 0; j < s; j++) {
            if(l[i][j] == '.') {
                l[i][j] = 'o';
                check(l);
                l[i][j] = '.';
            }
            else {
                check(l);
            }
        }         
    }
    std::cout << m/2 << std::endl;
}
