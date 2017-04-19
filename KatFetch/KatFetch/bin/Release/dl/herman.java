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

            int ta = int.Parse(Console.ReadLine());

            Console.WriteLine((Math.PI * Math.Pow(ta, 2)));

            Console.WriteLine((2 * Math.Pow(ta, 2)));

        }

    }

}