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

            int c = int.Parse(Console.ReadLine());

            int a = 1;

            int b = 0;

            for (int i = 0; i < c; i++)

            {

                int tmp = a;

                a = b;

                b += tmp;

            }

            Console.WriteLine(a + " " + b);

        }



    }

}