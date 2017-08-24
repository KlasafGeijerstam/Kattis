#include <iostream>
#include <string>
#include <algorithm>
#include <cstdlib>
#include <unordered_set>
#include <unordered_map>
#include <vector>

std::vector<int>* map[10];
std::unordered_set<int> val;
void rec(int n,std::string tot)
{
    if (stoi(tot + (char)('0' + n)) > 400)
        return;
    if (tot.size() > 4 && tot[4] == '0' && tot[3] == '0')
        return; 
    tot += (char)('0' + n);
    val.insert(stoi(tot));
    for (const auto& v : *map[n]) 
        rec(v, tot);
}

int closest(int num, std::vector<int> n) 
{
    int curr = n[0];
    for (const auto& c : n)
        if (abs(num - c) < abs(num - curr))
            curr = c;
    return curr;
}

void setup() 
{
    map[0] = new std::vector<int>();
    map[0]->push_back(0);
    map[1] = new std::vector<int>();
    map[1]->push_back(1);
    map[1]->push_back(2);
    map[1]->push_back(3);
    map[1]->push_back(4);
    map[1]->push_back(5);
    map[1]->push_back(6);
    map[1]->push_back(7);
    map[1]->push_back(8);
    map[1]->push_back(9);
    map[1]->push_back(0);
    map[2] = new std::vector<int>();
    map[2]->push_back(2);
    map[2]->push_back(3);
    map[2]->push_back(5);
    map[2]->push_back(6);
    map[2]->push_back(8);
    map[2]->push_back(9);
    map[2]->push_back(0);
    map[3] = new std::vector<int>();
    map[3]->push_back(3);
    map[3]->push_back(6);
    map[3]->push_back(9);
    map[4] = new std::vector<int>();
    map[4]->push_back(4);
    map[4]->push_back(5);
    map[4]->push_back(6);
    map[4]->push_back(7);
    map[4]->push_back(8);
    map[4]->push_back(9);
    map[4]->push_back(0);
    map[5] = new std::vector<int>();
    map[5]->push_back(5);
    map[5]->push_back(6);
    map[5]->push_back(8);
    map[5]->push_back(9);
    map[5]->push_back(0);
    map[6] = new std::vector<int>();
    map[6]->push_back(6);
    map[6]->push_back(9);
    map[7] = new std::vector<int>();
    map[7]->push_back(7);
    map[7]->push_back(8);
    map[7]->push_back(9);
    map[7]->push_back(0);
    map[8] = new std::vector<int>();
    map[8]->push_back(8);
    map[8]->push_back(9);
    map[8]->push_back(0);
    map[9] = new std::vector<int>();
    map[9]->push_back(9);
}

int main()
{
    setup();
    std::ios_base::sync_with_stdio(false);
    for (int i = 0; i < 10; i++)
        rec(i, "");
    std::cout << val.size() << std::endl << std::endl;
    val.erase(0);
    std::unordered_map<int, int> dic;
    std::vector<int> pam;
    pam.insert(pam.begin(), val.begin(), val.end());
    std::sort(pam.begin(), pam.end());
    for (int i = 1; i < 201; i++)
        dic[i] = closest(i, pam);
    for (const auto& v : dic) 
         std::cout << "dic[" << v.first << "] = " << v.second << ";" << std::endl;
    int p;
    std::cin >> p;
    
}
