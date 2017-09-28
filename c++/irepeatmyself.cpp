#include <iostream>
#include <string>

int main()
{
    std::ios_base::sync_with_stdio(false);

    int cases;
    std::string m;
    std::getline(std::cin, m);
    cases = std::stoi(m);

    for (int cc = 0; cc < cases; cc++)
    {
        std::string input,substr;
        std::getline(std::cin, input);
        int i = 0;
        for (; i < input.length(); i++)
        {
            substr += input[i];
            bool failed = false;
            for (int j = i + 1; j < input.length(); j++)
            {
                if (input[j] != substr[j%substr.length()]) {
                    failed = true;
                    break;
                }
            }
            if (!failed) {
                std::cout << i + 1 << std::endl;
                break;
            }
        }
        if(i == input.length())
            std::cout << input.length() << std::endl;
    }
}
