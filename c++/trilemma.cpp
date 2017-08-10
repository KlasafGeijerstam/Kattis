#include <algorithm>
#include <iostream>
#include <cmath>
#define N 1.57079632679489661923132
#define P 3.141592653589793
double dist(int X1, int Y1, int X2, int Y2)
{
    return sqrt(pow(X1 - X2, 2) + pow(Y1 - Y2, 2));
}

int main()
{
    int t, X1, Y1, X2, Y2, X3, Y3, e;
    double a, l1, l2, l3, v1, v2, v3;
    std::cin >> t;
    for (int i = 1; i <= t; i++) {
        std::cin >> X1 >> Y1 >> X2 >> Y2 >> X3 >> Y3;
        a = ((X2 - X1)*(Y3 - Y1) - (Y2 - Y1)*(X3 - X1)) / 2.0;
        std::cout << "Case #" << i << ": ";
        if (a != 0)
        {
            l1 = dist(X3, Y3, X1, Y1);
            l2 = dist(X2, Y2, X3, Y3);
            l3 = dist(X1, Y1, X2, Y2);
            
            v1 = acos(((X1 - X2)*(X1 - X3) + (Y1 - Y2)*(Y1 - Y3))/(l1*l3));
            v2 = acos(((X2 - X1)*(X2 - X3) + (Y2 - Y1)*(Y2 - Y3)) / (l2*l3));
            v3 =  P - v1 - v2;
            e = 0;
            if (l1 == l2)
                e++;
            if (l1 == l3)
                e++;
            if (l2 == l3)
                e++;
            if (e)
                std::cout << "isosceles ";
            else
                std::cout << "scalene ";
            
            if (v1 < N && v2 < N && v3 < N)
                std::cout << "acute ";
            else if (v1 > N || v2 > N || v3 > N)
                std::cout << "obtuse ";
            else
                std::cout << "right ";
            std::cout << "triangle" << std::endl;
        }
        else
            std::cout << "not a triangle" << std::endl;
    }
}
