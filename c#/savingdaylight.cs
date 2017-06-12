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

            string s;

            while((s = Console.ReadLine()) != null)

            {

                var l = s.Split();

                var g = new DateTime(1,1,1, int.Parse(l[3].Split(':')[0]), int.Parse(l[3].Split(':')[1]),0);

                var d = new DateTime(1, 1, 1, int.Parse(l[4].Split(':')[0]), int.Parse(l[4].Split(':')[1]), 0);

                var c = d - g;

                Console.WriteLine($"{l[0]} {l[1]} {l[2]} {c.Hours} hours {c.Minutes} minutes");

            }

        }

    }

} 