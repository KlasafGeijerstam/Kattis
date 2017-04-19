using System;

using System.Linq;

using System.Globalization;

using System.Collections.Generic;

using System.IO;



namespace Kattis

{

    class Program

    {

        static void Main(string[] args)

        {

            var p = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();

            for (int i = 1; i <= p[2]; i++)

            {

                if(i % p[0] != 0 && i % p[1] != 0)

                    Console.Write(i);

                else if(i % p[0] == 0)

                    Console.Write("Fizz");

                if (i % p[1] == 0)

                    Console.Write("Buzz");

                Console.Write("\n");

            }

        }

    }

}