#import <iostream>



using namespace std;



int main(){

    long p;

    cin >> p;

    if(p&1){

        cout << ((p+1)*(p+3))/4 << endl;

    }

    else

        cout << (p/2+1)*(p/2+1) << endl;



return 0;

}