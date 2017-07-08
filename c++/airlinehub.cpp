#include <iostream>
#include <string>
#include <stdlib.h>
#include <cmath>
#include <vector>
#include <float.h>
#include <algorithm>

#define PI 3.1415926535897932384626433832795028841971693993751058209749445923078164

using namespace std;

typedef struct pos {
    double lat, lng;
    bool operator==(const pos& r)
    {
        return r.lat == lat && r.lng == lng;
    }
} pos;

double haversine(double lat1,double long1,double lat2,double long2) 
{
    return pow(sin((lat2 - lat1) / 2), 2) + cos(lat1)*cos(lat2)*pow(sin((long2 - long1) / 2), 2);
}

double c(double lat1, double long1, double lat2, double long2)
{
    return 2 * atan2(sqrt(haversine(lat1, long1, lat2, long2)), sqrt(1 - haversine(lat1, long1, lat2, long2)));
}

double latDist(double lat1, double long1, double lat2, double long2) {
    return 6372.797560856 * c(lat1, long1, lat2, long2);
}

double toRad(double deg) 
{
    return deg*PI/180.0;
}

int main() 
{
    int n;

    while (scanf("%d",&n) == 1) 
    {
        double lat, lng, min = DBL_MAX, maxd = 0;
        pos m;
        vector<pos> vect = {};
        for (auto i = 0; i < n; i++)
        {
            scanf("%lf %lf", &lat, &lng);
            vect.push_back({lat,lng});
        }
        for (int i1 = 0; i1 < vect.size(); i1++)
        {
            maxd = 0;
            for (int i2 = 0; i2 < vect.size(); i2++)
            {
                if (i1 == i2)
                    continue;

                maxd = max(maxd,latDist(toRad(vect[i1].lat), toRad(vect[i1].lng), toRad(vect[i2].lat), toRad(vect[i2].lng)));
            }
            if (maxd <= min) 
            {
                m = vect[i1];
                min = maxd;
            }
        }
        printf("%.2lf %.2lf\n", m.lat, m.lng);
    }
}
