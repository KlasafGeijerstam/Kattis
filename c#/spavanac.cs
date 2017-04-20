using System;
using System.Linq;
using System.Globalization;
using System.Collections.Generic;

namespace Kattis
{
    class Program
    {
        static void Main(string[] args)
        {
            var l = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();
            DateTime dt = new DateTime(20,2,2, l[0], l[1], 0);
            dt = dt.Subtract(new TimeSpan(0, 45, 0));
            Console.WriteLine(dt.Hour + " " + dt.Minute);
        }
    }
}
