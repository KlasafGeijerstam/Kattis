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

            int num = 0;

            while (true)

            {

                num++;

                Dictionary<string, int> d = new Dictionary<string, int>();

                int ta = int.Parse(Console.ReadLine());

                if (ta == 0)

                    break;

                for (int i = 0; i < ta; i++)

                {

                    var s = Console.ReadLine().Split(' ');

                    if (d.ContainsKey(s[s.Length - 1].ToLower()))

                        d[s[s.Length - 1].ToLower()]++;

                    else

                        d[s[s.Length - 1].ToLower()] = 1;

                }

                Console.WriteLine($"List {num}:");

                List<string> sls = new List<string>();

                foreach (var kv in d)

                {

                    sls.Add($"{kv.Key} | {kv.Value}");

                }

                sls.Sort();

                foreach (var s in sls)

                {

                    Console.WriteLine(s);

                }

            }

        }

    }

}