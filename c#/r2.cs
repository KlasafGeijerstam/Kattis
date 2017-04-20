using System;
using System.Linq;
using System.Globalization;
using System.Collections.Generic;
using System.IO;

namespace Kattis
{
    class Program
    {
        public static void Main(string[] args)
        {
            var c = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();
            int r = c[0];
            int s = c[1];
            for (int i = -1000; i < 1001; i++)
            {
                if ((r + i) / 2 == s)
                {
                    Console.WriteLine(i);
                    break;
                }
            }
        }
    }
}
