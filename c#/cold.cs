using System;

using System.Collections.Generic;

using System.Linq;



namespace Kattis

{

    class Program

    {

        static void Main(string[] args)

        {

            Console.ReadLine();

            Console.WriteLine(Console.ReadLine().Split(' ').ToList().Where(x => int.Parse(x) < 0).Count());

        }

    }

}