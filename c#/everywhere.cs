using System;
using System.Collections.Generic;
using System.Linq;

namespace Kattis
{
    class Program
    {
        static void Main(string[] args)
        {
            List<string> t = new List<string>();
            int cases = int.Parse(Console.ReadLine());
            for (int i = 0; i < cases; i++)
            {
                t.Clear();
                int trips = int.Parse(Console.ReadLine());
                for (int j = 0; j < trips; j++)
                {
                    string c = Console.ReadLine();
                    if (!t.Contains(c))
                        t.Add(c); 
                }
                Console.WriteLine(t.Count);
            }
        }
    }
}