#include <iostream>

int main() {
    std::ios_base::sync_with_stdio(false);
    int d[9];
    for(int i = 0; i < 9; i++)
        std::cin >> d[i];
        
    for(int i = 0; i < 9; i++)
        for(int j = i + 1; j < 9; j++)
            for(int k = j + 1; k < 9; k++)
                for(int l = k + 1; l < 9; l++)
                    for(int m = l + 1; m < 9; m++)
                        for(int n = m + 1; n < 9; n++)
                            for(int o = n + 1; o < 9; o++)
                                if(d[i] + d[j] + d[k] + d[l] + d[m] + d[n] + d[o] == 100)
                                    std::cout << d[i] << std::endl << d[j] << std::endl << d[k] << std::endl << d[l] << std::endl << d[m] << std::endl << d[n] << std::endl << d[o] << std::endl;
                                
    
}
