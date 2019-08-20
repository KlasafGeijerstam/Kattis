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
            var s = "";
            while((s = Console.ReadLine()) != null)
            {
                var n = BigInteger.Parse(s);
                var ai = 1 + BigInteger.Parse(s)/2;
                while (BigInteger.Pow(ai, 3) > n)
                    ai = (2 * ai + n / BigInteger.Pow(ai, 2)) / 3;

                if (BigInteger.Abs(n - BigInteger.Pow(ai, 3)) > BigInteger.Abs(n - BigInteger.Pow(ai + 1, 3)))
                    ai += 1;
                Console.WriteLine(ai);
            }
        }
    }
}
