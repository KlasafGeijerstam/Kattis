#include <iostream>

using namespace std;



int main(){

    

    int cases;

    cin >> cases;

    for(int p = 0; p < cases; p++){

        int pcount;

        cin >> pcount;

        int arr[pcount+1];

        for(int i = 0; i < pcount; i++)

            cin >> arr[i];

        

        int moves = 0;

        int current = 1;

        

        for(int i = 0; i <= pcount; i++){

            if(arr[i] != current)

                moves++;

            else

                current++;

        }

    

        cout << moves-1 << endl;

    

    

    }

    

    return 0;

    

}