using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Threading;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            var p = Console.ReadLine().Split().Select(x => int.Parse(x)).ToArray();
            var h = Console.ReadLine().Split().Select(x => float.Parse(x)).ToArray();
            var poop = float.MaxValue;
            for (var i = 0; i < 3; i++)
                poop = Math.Min(p[i] / h[i],poop);
            for (int i = 0; i < 3; i++)
                Console.Write(Math.Round(p[i]- h[i]*poop,4) + " ");
        }
    }
}
