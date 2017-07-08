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
            while (true) {
               var x = Console.ReadLine().Split().Select(y => int.Parse(y)).ToArray();

                if (x[0] == 0 && x[1] == 0)
                    break;
                else if (x[0] + x[1] == 13)
                    Console.WriteLine("Never speak again.");
                else if (x[0] > x[1])
                    Console.WriteLine("To the convention.");
                else if (x[0] < x[1])
                    Console.WriteLine("Left beehind.");
                else
                    Console.WriteLine("Undecided.");
            }
        }
    }
}
