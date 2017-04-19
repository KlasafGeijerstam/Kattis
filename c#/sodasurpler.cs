using System;

using System.Linq;



namespace Kattis

{

    class Program

    {

        public static void Main(string[] args)

        {

            var s = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();

            var tot = s[0] + s[1];

            int count = 0;

            while ((tot -= s[2]) >= 0)

            {

                count++;

                tot++;

            }

            Console.WriteLine(count);

            Console.ReadLine();

        }

    }

}