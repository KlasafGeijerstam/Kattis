#include <iostream>
#include <bitset>
#include <algorithm>

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::bitset<100000001> b(0);
    b.set(1);
    int n, q, m;
    std::cin >> n >> q;
    m = n - 1;
    for(int i = 2; i <= sqrt(n) + 1; i++)
    {
        for(int j = i*i; j <= n; j += i)
        {
            if(!b.test(j))
                m--;
            b.set(j);
           
        }
    }
    std::cout << m << std::endl;
    for(int i = 0; i < q; i++)
    {
        std::cin >> n;
        printf("%c\n",(b.test(n) ? '0' : '1'));
    }
}
