#include <iostream>
#include <string>

using namespace std;

int main()
{
    while (true) 
    {
        int l,v1,v2;
        string s;
        cin >> l;
        if (l == 0)
            break;
        bool set[32]{};
        bool reg[32]{};
        for (int i = 0; i < l; i ++)
        {
            cin >> s;
            if (s == "SET")
            {
                cin >> v1;
                set[v1] = true;
                reg[v1] = 1;
            }
            else if (s == "CLEAR")
            {
                cin >> v1;
                set[v1] = true;
                reg[v1] = 0;
            }
            else if (s == "AND")
            {
                cin >> v1 >> v2;
                if (set[v1] && set[v2])
                    reg[v1] = reg[v1] && reg[v2];
                else if ((set[v1] && !reg[v1]) || (set[v2] && !reg[v2])) 
                {
                    reg[v1] = false;
                    set[v1] = true;
                }
                else
                    set[v1] = false;
            }
            else if (s == "OR") 
            {
                cin >> v1 >> v2;
                if (set[v1] || set[v2])
                    if ((set[v1] && !reg[v1] && !set[v2]) || (set[v2] && !reg[v2] && !set[v1]))
                        set[v1] = false;
                    else 
                    {
                        reg[v1] = (reg[v1] && set[v1]) || (reg[v2] && set[v2]);
                        set[v1] = true;
                    }  
                else
                    set[v1] = false;
            }
        }
        for (int i = 32 - 1; i >= 0; i--)
            cout << (set[i] ? to_string(reg[i]) : "?");
        cout << endl;
    }
}
