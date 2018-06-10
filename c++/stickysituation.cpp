#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;

int main()
{
    ios_base::sync_with_stdio(false);
    int n;
    cin >> n;
    if(n > 90) {
        cout << "possible" << endl; //Works without, 0.01 slower
    } else {
        vector<long long int> ns(n);
        while(n--)
            cin >> ns[n];
        sort(ns.begin(), ns.end());
        for(auto i = 0; i < ns.size()-2; i++){
            if((ns[i] < ns[i+1] < ns[i+2]) && (ns[i] + ns[i+1] > ns[i+2])) {
                cout << "possible" << endl;
                return 0;
            }
        }
        cout << "impossible" << endl;
    }
}
