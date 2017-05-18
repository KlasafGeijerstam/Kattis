#include <iostream>
#include <string>
#include <map>

using namespace std;

int main()
{
    int cases;
    cin >> cases;
    
    for (size_t i = 0; i < cases; i++)
    {
        map<int, int> mp;
        int ppl;
        cin >> ppl;
        for (size_t j = 0; j < ppl; j++)
        {
            int pep;
            cin >> pep;
            mp[pep]++;
            if (mp[pep] > 1)
                mp.erase(pep);
        }
        cout << "Case #" << i+1 << ": " << (*mp.begin()).first << endl;
    }
    return 0;
}