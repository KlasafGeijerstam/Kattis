#include<set>
#include<iostream>
#include<vector>

int main()
{
    int g;
    std::cin >> g;
    std::vector<int> s(g);

    while (g--) 
        std::cin >> s[g];
    
    std::set<int> c;
    for (int i = 1; i < 10E6; i++) {
        for (auto t : s)
            if (!c.insert(t % i).second)
                goto lel;
        std::cout << i << std::endl;
        break;
    lel:
        c.clear();
    }
    return 0;
}
