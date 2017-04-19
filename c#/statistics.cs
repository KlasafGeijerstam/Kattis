using System;

using System.Linq;

using System.Numerics;

using System.Collections.Generic;

using System.IO;

using System.Diagnostics;



namespace Kattis

{



    public class Program

    {

        public static void Main()

        {

            string str;

            int cc = 1;

            while((str = Console.ReadLine()) != null)

            {

                Console.WriteLine($"Case {cc}: {str.Split(' ').Skip(1).Select(x => int.Parse(x)).ToArray().Min()} {str.Split(' ').Skip(1).Select(x => int.Parse(x)).ToArray().Max()} {(str.Split(' ').Skip(1).Select(x => int.Parse(x)).ToArray().Count() > 1 ? (str.Split(' ').Skip(1).Select(x => int.Parse(x)).ToArray().Max() - str.Split(' ').Skip(1).Select(x => int.Parse(x)).ToArray().Min()) : 0)}");

                cc++;

            }

        }

    }

}