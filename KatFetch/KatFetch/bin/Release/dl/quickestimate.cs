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

            var i = int.Parse(Console.ReadLine());

            for (int j = 0; j < i; j++)

            {

                Console.WriteLine(Console.ReadLine().Length);

            }

        }

    }

}