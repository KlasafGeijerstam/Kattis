using System;

using System.Linq;

using System.Globalization;

using System.Collections.Generic;

using System.IO;



namespace Kattis

{

    class Program

    {

        public static void Main(string[] args)

        {

            

            int c = int.Parse(Console.ReadLine());

            for (int i = 0; i < c; i++)

            {

                int p = int.Parse(Console.ReadLine());

                var d = new Dictionary<String, int>();

                for (int j = 0; j < p; j++)

                {

                    var k = Console.ReadLine().Split(' ');

                    if (d.ContainsKey(k[1]))

                        d[k[1]]++;

                    else

                        d.Add(k[1], 2);

                }

                if(d.Count == 0)

                    Console.WriteLine(0);

                else

                {

                    long tot = 1;

                

                    foreach (var kv in d)

                    {

                        tot *= kv.Value;

                    }

                    Console.WriteLine(tot - 1);

                }

            }

        }

    }

}