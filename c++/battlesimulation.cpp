#include <iostream>

using namespace std;
int main()
{
    string s;
    cin >> s;
    for (size_t i = 0; i < s.length(); i++)
    {
        if (i + 2 < s.length() && s[i] + s[i + 1] + s[i + 2] == 224) {
            cout << "C";
            i += 2;
        }
        else
            if (s[i] == 'R')
                cout << "S";
            else if (s[i] == 'B')
                cout << "K";
            else
                cout << "H";
    }
}
