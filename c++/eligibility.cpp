#include <algorithm>
#include <iostream>
#include <map>
#include <string>
#include <iomanip>

int main()
{
    std::ios_base::sync_with_stdio(false);
    int c,co;
    std::string s, y, st;
    std::cin >> c;
    while (c)
    {
        std::cin >> s >> y >> st >> co;
        y = y.substr(0, 4);
        st = st.substr(0, 4);
        if (std::stoi(y) > 2009 || std::stoi(st) > 1990)
        {
            std::cout << s << " eligible" << std::endl;
        }
        else if(co > 40)
            std::cout << s << " ineligible" << std::endl;
        else
            std::cout << s << " coach petitions" << std::endl;
        c--;
    }
    std::cin >> c;
} 
