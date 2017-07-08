#include <iostream>
#include <string>

using namespace std;

int main()
{
while (true)
	{
		int cases;
		cin >> cases;

		if (!cases)
			break;
		string fav;
		int pairs = -1;
		for (size_t i = 0; i < cases; i++)
		{
			string p;
			cin >> p;
			char prev = p[0];
			int lp = 0;
			for (size_t j = 1; j < p.size(); j++)
			{
				if (prev == p[j] && (prev == 'a' || prev == 'e' || prev == 'i' || prev == 'o' || prev == 'u' || prev == 'y'))
					lp++;
				prev = p[j];
			}
			if (lp > pairs) {
				fav = p;
				pairs = lp;
			}
		}
		cout << fav << endl;
	}
	return 0;
}
