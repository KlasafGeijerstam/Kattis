using System;

using System.Collections.Generic;

using System.IO;

using System.Linq;

using System.Numerics;

using System.Globalization;



namespace Kattis

{

    class Program

    {



        static void Main(string[] args)

        {



            var ta = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToList();

            ta[0]--;

            double d = 0;

            int g = ta[1] / ta[2];



            d = ta[0] / (ta[1] / ta[2]);

            if (ta[0] % (ta[1] / ta[2]) > 0)

                d++;



            Console.WriteLine(d);

        }

    }

}