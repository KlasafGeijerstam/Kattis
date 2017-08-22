#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
//Solved using Knuth-Morris-Pratt
//https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm


std::string s, w;
std::vector<int> t(1000000);
int pm;

bool match(int i)
{
    int c = 0, o = i;
    for (; i < o + w.size(); i++, c++)
        if (s[i] != w[c])
            return false;
    return true;
}

void buildTable()
{
    int pos = 1, cnd = 0;
    t[0] = -1;
    while (pos < w.length()) 
    {
        if (w[pos] == w[cnd])
        {
            t[pos] = t[cnd];
            pos++;
            cnd++;
        }
        else
        {
            t[pos] = cnd;
            cnd = t[cnd];
            while (cnd >= 0 && w[pos] != w[cnd]) 
                cnd = t[cnd];
            pos++;
            cnd++;
        }
    }
    t[pos] = cnd;
}

void kmp()
{
    int m = 0, i = 0;
    while (m + i < s.length())
    {
        if (w[i] == s[m + i])
        {
            i++;
            if (i == w.length())
            {
                std::cout << m << " ";
                m = m + i - t[i];
                i = t[i];
            }
        }
        else
        {
            if (t[i] > -1) 
            {
                m = m + i - t[i];
                i = t[i];
            }
            else
            {
                m = m + i + 1;
                i = 0;
            }
        }
    }
}

int main()
{
    std::ios_base::sync_with_stdio(false);
    while ((std::getline(std::cin,w)))
    {
        std::getline(std::cin, s);
        buildTable();
        kmp();
        std::cout << std::endl;
    }
}
