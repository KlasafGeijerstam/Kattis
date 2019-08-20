using System;

namespace Kattis
{
    class Program
    {
        static void Main(string[] args)
        {
            int num;
            int max = 1001;
            int min = 0;
            while (true)
            {
                num = min + ((max-min) / 2);
                Console.WriteLine(num);
                switch (Console.ReadLine())
                {
                    case "lower":
                        max = num;
                        break;
                    case "higher":
                        min = num;
                        break;
                    case "correct":
                        return;
                }
            }
        }
    }
}
