using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Threading;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            int[] vals = { 0, 1000, 12, 3, 22, 10, 8, 3 };
            var lev = new Dictionary<string, int>() { {"th",0 }, { "in",1 }, { "ft", 2}, { "yd", 3}, { "ch", 4}, { "fur", 5}, { "mi",6 }, { "lea",7 } };
            var l = Console.ReadLine().Split();
            var val = double.Parse(l[0]);
            l[3] = From(l[3]);
            l[1] = From(l[1]);
            if (lev[l[1]] > lev[l[3]])
            {
                var cur = lev[l[1]];
                var tar = lev[l[3]];
                do
                    val *= vals[cur--];
                while (cur != tar);
                Console.WriteLine(val);
            }
            else if (lev[l[1]] < lev[l[3]])
            {
                var cur = lev[l[1]];
                var tar = lev[l[3]];
                do
                    val /= vals[++cur];
                while (cur != tar);
                Console.WriteLine(val);
            }
            else
                Console.WriteLine(val);
        }
        static string From(string s)
        {
            if (s.Length < 4)
                return s;
            switch (s)
            {
                case "thou":
                    return "th";
                case "inch":
                    return "in";
                case "foot":
                    return "ft";
                case "yard":
                    return "yd";
                case "chain":
                    return "ch";
                case "furlong":
                    return "fur";
                case "mile":
                    return "mi";
                case "league":
                    return "lea";
            }
            return "";
        }
    }
}
