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
            var x = Console.ReadLine().Split().Select(p => int.Parse(p)).ToArray();
            var bob = (x[1] - x[0]) * x[2];
            var alice = 0;
            while (alice <= bob)
                alice += x[4];
            Console.WriteLine(alice / x[4] + x[3]);
        }
    }
}

