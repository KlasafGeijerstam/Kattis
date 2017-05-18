using System;

using System.Linq;



namespace Kattis

{

    public class Program

    {

        public static void Main()

        {

            int cases = int.Parse(Console.ReadLine());

            for (int i = 0; i < cases; i++)

            {

                int[] p = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();

                var t = (p[1] % (Math.Pow(2,p[0]))) == Math.Pow(2,p[0])-1 ? "ON" : "OFF";

                Console.WriteLine($"Case #{i+1}: {t}");

            }

        }

    }

}