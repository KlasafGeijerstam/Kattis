using System;
using System.Collections.Generic;
using System.Linq;

namespace Kattis
{
    class Program
    {
        static void Main(string[] args)
        {
            List<int> m = new List<int>();
            for (int i = 0; i < 5; i++)
            {
                int s = 0;
                Console.ReadLine().Split(' ').ToList().ForEach(x => s += int.Parse(x));
                m.Add(s);
            }
            Console.WriteLine(m.IndexOf(m.Max())+1 + " " + m.Max());
            Console.ReadLine();
        }
    }
}
