#include <iostream>
#include <map>
#include <vector>

struct V {
    int x, y;
};

int main()
{
    int n, x, y, max,lm;

    while (true)
    {
        std::cin >> n;
        if (!n)
            break;
        max = 0,lm = 0;
        std::map<double, int> k;
        std::vector<V> p(n);
        for (int i = 0; i < n; i++)
        {
            std::cin >> x >> y;
            p[i] = { x,y };
        }

        for (int i = 0; i < n; i++)
        {
            for (int j = i+1; j < n; j++)
            {
                double d1 = (double)p[i].x - p[j].x, d2 = (double)p[i].y - p[j].y, div;
                
                if (d2 == 0)
                    div = 1337;
                else
                    div = d1 / d2;

                if (++(k[div]) > max)
                    max = k[div];
            }
            if(max > lm)
                lm = max;
            k.clear();
        }
        std::cout << lm+1 << std::endl;
    }
} 
