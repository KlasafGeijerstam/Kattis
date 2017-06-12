using System;

using System.Linq;



namespace Kattis

{

    public class Program

    {

        public static void Main()

        {

            Console.WriteLine(Enumerable.Range(0, int.Parse(Console.ReadLine())).Aggregate(2, (x, y) => x * 2 - 1, x => x * x));

        }

    }

}