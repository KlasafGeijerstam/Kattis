#include <iostream>

#include <string>

#include <stdlib.h>



using namespace std;

int arr[1000][1000];

int r, c;

void find(int y, int x, int clor, int rig) 

{

    if (y < 0 || x < 0 || y >= r || x >= c || arr[y][x] != rig) return;

    arr[y][x] = clor;

    find(y - 1, x, clor, rig);

    find(y, x - 1, clor, rig);

    find(y + 1, x, clor, rig);

    find(y, x + 1, clor, rig);

}



int main() 

{

    int n,r1,c1,r2,c2;

    cin >> r >> c;

    

    for (auto i = 0; i < r; i++) 

    {

        string s;

        cin >> s;

        for (auto j = 0; j < s.length(); j++)

            arr[i][j] = s[j];

    }

    int plor = 0;

    int flor = -1;

    for (int i = 0; i < r; i++)

        for (int j = 0; j < c; j++)

            if (arr[i][j] == '0' || arr[i][j] == '1')

                find(i, j, arr[i][j] == '0' ? plor -= 2 : flor -= 2, arr[i][j]);



    cin >> n;



    for (auto i = 0; i < n; i++) 

    {

        cin >> r1 >> c1 >> r2 >> c2;

        if (arr[r1-1][c1-1] == arr[r2-1][c2-1]) {

            string str = (abs(arr[r1-1][c1-1]) % 2 == 0 ? "binary" : "decimal");

            cout << str << endl;

        }

        else

            cout << "neither" << endl;

    }

}