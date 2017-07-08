using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            var len = int.Parse(Console.ReadLine());
            for (int i = 0; i < len; i++)
            {
                Console.ReadLine();
                var x1 = Console.ReadLine().Split().Select(x => BigInteger.Parse(x)).OrderBy(x => x).ToList();
                var x2 = Console.ReadLine().Split().Select(x => BigInteger.Parse(x)).ToList().OrderByDescending(x => x).ToList();
                var sum = BigInteger.Zero;
                for (int k = 0; k < x1.Count; k++)
                    sum += x1[k] * x2[k];
                Console.WriteLine($"Case #{i+1}: {sum}");
            }         
        }
    }
}
