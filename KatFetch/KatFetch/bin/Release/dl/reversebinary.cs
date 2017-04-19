using System;

using System.Collections.Generic;

using System.IO;

using System.Linq;

using System.Numerics;

namespace Kattis

{

    class Program

    {



        static void Main(string[] args)

        {

            string s = Convert.ToString(int.Parse(Console.ReadLine()),2);

            long t = 0;

            for (int i = s.Length-1; i >= 0 ; i--)

            {

                if (s[i] == '1')

                    t += (long)Math.Pow(2, i);

            }

            Console.WriteLine(t);

            Console.ReadKey();

        }

    }

}