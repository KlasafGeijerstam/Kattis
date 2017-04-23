#include <iostream>
#include <map>

using namespace std;

int main(){
    
    int cases;
    cin >> cases;
    for(int p = 0; p < cases; p++){
        int pcount;
        cin >> pcount;
        map<string,int**> myMap;
        for(int i = 0; i < pcount; i++){
            string p1;
            string p2;
            cin >> p1;
            cin >> p2;
            
            if(myMap.count(p1) == 0){
               int* p = (int*)malloc(sizeof(int));
               *p = 1;
               myMap[p1] = &p;
            }
            if(myMap.count(p2) == 0){
               int* p = (int*)malloc(sizeof(int));
               *p = 1;
               myMap[p2] = &p;
            }
            if((*(*myMap[p1])) > (*(*myMap[p2]))){
                (*(*myMap[p1])) += (*(*myMap[p2]));
                (*myMap[p2]) = (*myMap[p1]); 
            }
            else{
                (*(*myMap[p2])) += (*(*myMap[p1]));
                (*myMap[p1]) = (*myMap[p2]); 
            }
            cout << (*(*myMap[p1])) << endl;
            
        }
    }
    return 0;
}
