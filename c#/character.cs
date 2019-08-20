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

            var p = new BigInteger(long.Parse(Console.ReadLine()));

            var result = BigInteger.Zero;

            for (int i = 2; i <= p; i++)

            {

                result += Choose(p, i);

            }

            Console.WriteLine(result);

        }

        

        static BigInteger Choose(BigInteger n,BigInteger k)

        {

            return Fac(n) / (Fac(k) * Fac(n - k));

        }



        static BigInteger Fac(BigInteger l)

        {

            if (l < 2)

                return 1;



            return l * Fac(l - 1);

        }

    }

}