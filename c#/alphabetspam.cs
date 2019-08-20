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
            int[] a = { 0, 0, 0, 0 };
            string s = Console.ReadLine();
            foreach (var c in s)
            {
                if (c == '_')
                    a[0]++;
                else if (c >= 'A' && c <= 'Z')
                    a[2]++;
                else if (c >= 'a' && c <= 'z')
                    a[1]++;
                else
                    a[3]++;
            }
            for (int i = 0; i < 4; i++)
                Console.WriteLine((float)a[i]/s.Length);
        }
    }
}
