using System;

using System.Linq;

using System.Numerics;

using System.Collections.Generic;

using System.IO;

using System.Diagnostics;

using System.Globalization;



namespace Kattis

{



    public class Program

    {

        public static void Main()

        {

            int cases = int.Parse(Console.ReadLine());

            for (int i = 0; i < cases; i++)

            {

                var s = Console.ReadLine();

                var io = (int)Math.Sqrt(s.Length);

                var str = "";

                for (int y = io-1; y >= 0; y--)

                {

                    for (int x = y; x < s.Length; x+=io)

                    {

                        Console.Write(s[x]);

                    }

                }

                Console.WriteLine();

            }

        }

    }

}