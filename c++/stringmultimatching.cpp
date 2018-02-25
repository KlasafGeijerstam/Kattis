#include <iostream>
#include <algorithm>
#include <vector>
#include <string>

int main()
{
	std::ios_base::sync_with_stdio(false);
	int n;
	std::string line;
	while (std::getline(std::cin, line)) {

		n = std::stoi(line);
		std::vector<std::string> pat(n);
		
		for (int i = 0; i < n; i++)
			std::getline(std::cin, pat[i]);	
		std::getline(std::cin,line);
		for (int i = 0; i < n; i++)
		{
			size_t ind = 0;
			while((ind != std::string::npos)){
				ind = line.find(pat[i], ind);
				if (ind != std::string::npos) {
					std::cout << ind << " ";
					ind++;
				}
			}
			std::cout << std::endl;
		}
	}
}
