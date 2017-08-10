#include <algorithm>
#include <iostream>
#include <map>
#include <string>
#include <iomanip>

struct vp {
    int w, l;
    vp():w(0),l(0){}
};

int w(std::string m1, std::string m2) 
{
    if (m1 == m2)
        return 0;
    if (m1 == "rock" && m2 == "paper" || m1 == "paper" && m2 == "scissors" || m1 == "scissors" && m2 == "rock")
        return 2;
    if (m2 == "rock" && m1 == "paper" || m2 == "paper" && m1 == "scissors" || m2 == "scissors" && m1 == "rock")
        return 1;
    return -1;
}

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::map<int, vp> pls;
    int n, k,p1,p2,cout = 0;
    std::string m1, m2;
    
    while (true)
    {
        pls.clear();
        cout++;
        std::cin >> n;
        if (n == 0)
            break;
        std::cin >> k;
        for (int i = 0; i < k*n*(n - 1) / 2; i++)
        {
            std::cin >> p1 >> m1 >> p2 >> m2;
            int r = w(m1, m2);
            if (r == 1)
            {
                pls[p1].w++;
                pls[p2].l++;
            }
            else if (r == 2)
            {
                pls[p2].w++;
                pls[p1].l++;
            }
        }
        if (cout != 1)
            std::cout << " " << std::endl;
        for (int i = 1; i <= n; i++)
        {
            vp v = pls[i];
            if (v.w + v.l == 0)
                std::cout << "-" << std::endl;
            else
                std::cout << std::fixed << std::setprecision(3) << (v.w * 1.0/ (v.w + v.l)) << std::endl;
        }
    }
}
