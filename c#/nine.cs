using System;

using System.Collections.Generic;

using System.Globalization;

using System.Linq;

using System.Text;

using System.Numerics;



namespace Kattis

{

    public class Program

    {

        public static void Main()

        {

            Enumerable.Range(0, int.Parse(Console.ReadLine())).ToList().ForEach(x => Console.WriteLine((8 * BigInteger.ModPow(9, long.Parse(Console.ReadLine())-1, 1000000007)) % 1000000007));

        }

    }

} 