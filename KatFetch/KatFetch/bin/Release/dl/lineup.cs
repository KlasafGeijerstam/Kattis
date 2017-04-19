using System;

using System.Collections.Generic;

using System.Linq;



namespace Kattis

{

    class Program

    {

        static void Main(string[] args)

        {

            int c = int.Parse(Console.ReadLine());

            List<string> org = new List<string>();

            List<string> dec = new List<string>();

            List<string> inc = new List<string>();

            for (int i = 0; i < c; i++)

            {

                org.Add(Console.ReadLine());

            }

            dec.AddRange(org);

            inc.AddRange(org);

            dec = dec.OrderByDescending(x => x).ToList();

            inc = inc.OrderBy(x => x).ToList();



            for (int i = 0; i < org.Count; i++)

            {

                if (org[i] != dec[i])

                    goto gat;

            }

            Console.WriteLine("DECREASING");

            return;

            gat:

            for (int i = 0; i < org.Count; i++)

            {

                if (org[i] != inc[i])

                    goto bat;

            }

            Console.WriteLine("INCREASING");

            return;

            bat:

            Console.WriteLine("NEITHER");

        }

    }

}