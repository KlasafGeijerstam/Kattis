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
            string sa;
            while ((sa = Console.ReadLine()) != null)
            {
                var ar = sa.Split(' ');
                string name = "";
                float tot = 0;
                int c = 0;
                foreach (var s in ar)
                {
                    if (!char.IsNumber(s[0]))
                        name += " " + s;
                    else
                    {
                        c++;
                        tot += float.Parse(s, CultureInfo.InvariantCulture);
                    }
                }
                Console.WriteLine((tot / c).ToString("F6", CultureInfo.InvariantCulture) + name); 
            }
        }
    }
}
