using System;

using System.Collections.Generic;

using System.Globalization;

using System.Linq;



namespace Kattis

{

    public class Program

    {

        public static void Main()

        {

            int n;

            bool fts = true;

            while ((n = int.Parse(Console.ReadLine())) != 0)

            {

                var c = new List<vl>();

                for (int i = 0; i < n; i++)

                {

                    var tmp = Console.ReadLine();

                    vl k;

                    k.mod = tmp.Split(' ')[1];

                    tmp = tmp.Replace(".", "");

                    k.dt = DateTime.ParseExact(tmp, tmp.Length == 7 ? "h:mm tt" : "hh:mm tt", CultureInfo.InvariantCulture);

                    c.Add(k);

                }

                c.Sort();

                if (fts)

                    fts = false;

                else

                    Console.WriteLine();

                c.ForEach(x =>

                {

                    Console.WriteLine(x);

                });

            }

        }

        struct vl:IComparable

        {

            public DateTime dt;

            public string mod;



            public int CompareTo(object obj)

            {

                return dt.CompareTo(((vl)obj).dt);

            }



            public override string ToString()

            {

                return dt.ToString("h:mm") + " " + mod;

            }

        }

    }

}