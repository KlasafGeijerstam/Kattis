using System;

using System.Linq;

using System.Globalization;

using System.Collections.Generic;



namespace Kattis

{

    class Program

    {

        static void Main(string[] args)

        {

            List<int> l = new List<int>();

            for (int i = 0; i < 10; i++)

            {

                var j = int.Parse(Console.ReadLine());

                if (!l.Contains(j % 42))

                    l.Add(j % 42);

            }

            Console.WriteLine(l.Count) ;



            Console.ReadLine();

        }

    }

}