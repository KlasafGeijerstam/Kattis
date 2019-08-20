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

            for (int i = int.Parse(Console.ReadLine()); i > 0; i--)

            {

                var l = Console.ReadLine().Split();

                var g = new DateTime(2,2,2,int.Parse(l[2]), int.Parse(l[3]),0);

                if(l[0] == "F")

                    g += new TimeSpan(0, int.Parse(l[1]), 0);

                else

                    g -= new TimeSpan(0, int.Parse(l[1]), 0);

                Console.WriteLine($"{g.Hour} {g.Minute}");

            }

        }

    }

} 