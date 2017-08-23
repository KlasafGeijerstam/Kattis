#include <iostream>
#include <string>
#include <algorithm>

int main()
{
    std::ios_base::sync_with_stdio(false);
    char oseat,seat;
    int f1 = 0, f2 = 0, f3 = 0;
    std::string s;
    std::cin >> s;
    oseat = seat = s[0];
    for (int i = 1; i < s.length(); i++)
        if (seat != s[i]) 
        {
            seat = s[i];
            f1++;
        }
        if (seat != 'U')
        {
            seat = 'U';
            f1++;
        }
    seat = oseat;
    for (int i = 1; i < s.length(); i++)
        if (seat != s[i])
        {
            seat = s[i];
            f2++;
        }
        if (seat != 'D')
        {
            seat = 'D';
            f2++;
        }
    seat = oseat;
    for (int i = 1; i < s.length(); i++)
        if (seat != s[i])
        {
            seat = s[i];
            f3++;
        }
    std::cout << f1 << std::endl << f2 << std::endl << f3 << std::endl;
}
