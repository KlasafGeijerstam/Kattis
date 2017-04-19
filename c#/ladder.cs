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

            var d = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToList();

            Console.WriteLine((long)Math.Ceiling(d[0] / (Math.Sin((Math.PI / 180) * d[1]))));

        }

    }

}