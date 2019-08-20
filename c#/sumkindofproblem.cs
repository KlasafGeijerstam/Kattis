using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Threading;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            for (int i = int.Parse(Console.ReadLine()); i > 0; i--)
            {
                var p = Console.ReadLine().Split().Select(x => int.Parse(x)).ToArray();
                var s1 = p[1] * (p[1] + 1) / 2;
                var s2 =  (int)Math.Ceiling(Math.Pow(p[1], 2));
                var s3 = p[1] * (p[1] + 1);
                Console.WriteLine($"{p[0]} {s1} {s2} {s3}");
            }
        }
    }
}
