#include <iostream>
#include <string>
#include <algorithm>
#include <vector>

int main()
{
    std::ios_base::sync_with_stdio(false);
    size_t n;
    int a, b, c, d, e;
    std::string sa, sb, sc, sd, se;
    std::cin >> n;
    for (size_t i = 0; i < n; i++)
    {
        std::cin >> sa >> sb >> sc >> sd >> se;
        a = (sa != "?" ? stoi(sa) : -1);
        b = (sb != "?" ? stoi(sb) : -1);
        c = (sc != "?" ? stoi(sc) : -1);
        d = (sd != "?" ? stoi(sd) : (a == -1 ? 0 : -1));
        e = (se != "?" ? stoi(se) : -1);
        
        for (int bm = (b == -1 ? 0 : b); bm <= (b == -1 ? 100 : b); bm++)
            for (int cm = (c == -1 ? 0 : c); cm <= (c == -1 ? 100 : c); cm++)
            {
                int am = a, dm = d, em = e;
                if (a == -1)
                    am = bm + cm + d;
                else if (d == -1)
                    dm = a - bm - cm;

                if (e == -1)
                    em = 3 * bm + cm;
                if(am == bm + cm + dm && 3*bm + cm == em && am >= 0 && am <= 100 && dm >= 0 && dm <= 100)
                    std::cout << am << " " << bm << " " << cm << " " << dm << " " << em << std::endl;
            }
    }
}
