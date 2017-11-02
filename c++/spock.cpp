#include <vector>
#include <string>
#include <iostream>

struct Rnd {
    char a, b, x;
};

inline int cac(Rnd r) {
    int x = (r.a*r.x + r.b) % 127;
    return x % 5;
}

Rnd upd(Rnd r) {
    r.x =  (r.a*r.x + r.b) % 127;
    return r;
}

int get(char c) {
    switch(c) {
        case 'r':
                return 0;
        case 'p':
                return 1;
        case 's':
                return 2;
        case 'l':
                return 3;
        case 'S':
                return 4;
    }
    return 1336;
}

const int prounds = 10;

int main() {
    std::ios_base::sync_with_stdio(false);
    std::vector<Rnd> rds(127*127*127);
    std::vector<Rnd> nws;
    std::string ans;
    int rounds;
    std::cin >> rounds;
    int mach = 0;
    for(char x = 0; x < 127; x++) {
        for(char a = 0; a < 127; a++) {
            for(char b = 0; b < 127; b++) {
                rds[mach++] = {a, b, x};
            }
        }
    }

    //first
    for(int i = 0; i < prounds; i++){
        std::cout << "Spock" << std::endl;
        std::cin >> ans;
        nws.clear();
        for(const auto& m : rds) {
            if(cac(m) == get(ans[0]))
                nws.push_back(upd(m));
        }
         
        rds = std::vector<Rnd>(nws);

    }

    for(int i = 0; i < rounds - prounds; i++) {
        int p = cac(rds[rds.size()-1]);
        
        if(p == 0)
            std::cout << "Spock" << std::endl;
        else if(p == 1)
            std::cout << "scissors" << std::endl;
        else if(p == 2)
            std::cout << "Spock" << std::endl;
        else if(p == 3)
            std::cout << "rock" << std::endl;
        else if(p == 4)
            std::cout << "lizard" << std::endl;

        std::cin >> ans;
        //Check if answered correct
        if(p != get(ans[0])) {
            //Wrong answer, see if any other machine would have gotten it
            for(int j = 0; j < rds.size() - 1; j++) {
                if(cac(rds[j]) == get(ans[0])) {
                    //Use this as the new balla machine
                    Rnd gm = rds[j];
                    rds.erase(rds.begin() + j);
                    rds[rds.size()-1] = gm;
                  
                    break;
                }
            }
        }
        for(int j = 0; j < rds.size(); j++) {
            rds[j] = upd(rds[j]);
        }
    }
}
