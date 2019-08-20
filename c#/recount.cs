using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Numerics;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            var i = new Dictionary<string, int>();
            string s;
            int max = -1;
            string mas = "";
            var dub = false;
            while((s = Console.ReadLine()) != "***")
            {
                if (!i.ContainsKey(s))
                    i.Add(s, 0);
                i[s]++;
                if (i[s] == max)
                    dub = true;
                if(i[s] > max)
                {
                    max = i[s];
                    dub = false;
                    mas = s;
                }
            }
            Console.WriteLine(dub ? "Runoff!" : mas);
        }
    }
} 
