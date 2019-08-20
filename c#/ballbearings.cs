using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Numerics;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            for (int i = int.Parse(Console.ReadLine()); i > 0; i--)
            {
                var t = Console.ReadLine().Split().Select(x => double.Parse(x,CultureInfo.InvariantCulture)).ToArray();
                Console.WriteLine((int)(Math.PI / Math.Asin((t[2] + t[1]) / (t[0] - t[1]))));
            }
        }
    }
} 
