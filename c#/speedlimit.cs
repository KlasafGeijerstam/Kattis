using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Kattis
{
    class Program
    {
        
        static void Main(string[] args)
        {
            int c;
            while ((c= int.Parse(Console.ReadLine())) > 0)
            {
                int di = 0;
                int prev = 0;
                for (int i = 0; i < c; i++)
                {
                    var ia = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();
                    di += (ia[1] - prev) * ia[0];
                    prev = ia[1];
                }
                Console.WriteLine(di + " miles");
            }
        }
    }
}
