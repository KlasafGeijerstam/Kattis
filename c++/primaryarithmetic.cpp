#include <iostream>
#include <string>

int main()
{
    while (true)
    {
        int m, k;
        std::cin >> m >> k;
        if (!(m + k))
            break;
        int m1[12] = { 0 }, m2[12] = { 0 };
        std::string s = std::to_string(m);
        for (int i = s.length() - 1; i >= 0; i--)
            m1[11-i] = s[s.length() - 1 - i] - '0';
        s = std::to_string(k);
        for (int i = s.length() - 1; i >= 0; i--)
            m2[11-i] = s[s.length() - 1 - i] - '0';
        int carry = 0;
        for (int i = 11; i > 0; i--)
        {
            if (m2[i] + m1[i] > 9)
            {
                carry++;
                m2[i] = (m2[i] + m1[i]) - 10;
                m1[i - 1]++;
            }
            else
                m2[i] += m1[i];

        }
        if (!carry)
            std::cout << "No carry";
        else
            std::cout << carry << " carry";
        if (carry > 1)
            std::cout << " operations." << std::endl;
        else 
            std::cout << " operation." << std::endl;
    }
}
