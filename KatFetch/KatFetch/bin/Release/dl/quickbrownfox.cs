using System;

using System.Collections.Generic;

using System.IO;

using System.Linq;

using System.Numerics;

namespace Kattis

{

    class Program

    {



        static void Main(string[] args)

        {

            char[] alph = "abcdefghijklmnopqrstuvwxyz".ToCharArray();

            int c = int.Parse(Console.ReadLine());

            for (int i = 0; i < c; i++)

            {

                Dictionary<char, int> d = new Dictionary<char, int>();

                for (int j = 0; j < alph.Length; j++)

                {

                    d.Add(alph[j], 0);

                }

                string p = Console.ReadLine();

                foreach (var ca in p)

                {

                    char key = ca.ToString().ToLower()[0];

                    if (d.ContainsKey(key))

                        d[key]++;

                }



                List<char> ot = new List<char>();

                foreach (var kv in d)

                {

                    if (kv.Value == 0)

                        ot.Add(kv.Key);

                }

                ot.Sort();

                

                if(ot.Count > 0)

                {

                    string s = "";

                    foreach (var mp in ot)

                    {

                        s += mp;

                    }

                    Console.WriteLine("missing " + s);

                }

                else

                    Console.WriteLine("pangram");

                 

            }



        }

    }

}