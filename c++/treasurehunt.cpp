#include<set>
#include<iostream>
#include<unordered_set>
#include<vector>
#include<string>

struct point {
    int x, y;
};

inline bool operator==(const point& a, const point& b) {
    return a.x == b.x && a.y == b.y;
}

namespace std {
    template <> struct hash<point> {
        size_t operator()(const point& o) const {
            return o.x * 7 + o.y;
        }
    };
}

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::unordered_set<point> ps;
    int r, c, x = 0, y = 0, k = 0;
    std::cin >> r >> c;
    std::vector<std::string> map(r);

    for (int i = 0; i < r; i++)
        std::cin >> map[i];
    
    while (map[y][x] != 'T') 
    {
        if (ps.find({ x, y }) != ps.end()) 
        {
            std::cout << "Lost" << std::endl;
            return 0;
        }
        ps.insert({ x,y });
        switch (map[y][x]) 
        {
            case 'N':
                y--;
                break;
            case 'S':
                y++;
                break;
            case 'W':
                x--;
                break;
            case 'E':
                x++;
                break;
            default:
                break;
        }
        k++;
        if (x < 0 || x >= c || y < 0 || y >= r) {
            std::cout << "Out" << std::endl;
            return 0;
        }
    }
    std::cout << k << std::endl;
    std::cin >> c;
    return 0;
}
