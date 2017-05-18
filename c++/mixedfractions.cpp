#include <iostream>

using namespace std;



int main()

{

    long p1,p2;

    while (cin >> p1 >> p2 && p1 != 0 && p2 != 0)

        cout << p1 / p2 << " " << p1%p2 << " / " << p2 << endl;

    return 0;

}