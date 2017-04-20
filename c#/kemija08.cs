using System;
using System.Linq;
using System.Globalization;
using System.Collections.Generic;
using System.IO;

namespace Kattis
{
    class Program
    {
        static void Main(string[] args)
        {
            var s = Console.ReadLine();
            for (int i = 0; i < s.Length; i++)
            {
                if(s[i] == 'a' || s[i] == 'e' || s[i] == 'i' || s[i] == 'o' || s[i] == 'u')
                {
                    s = s.Remove(i + 1, 2);
                }
            }
            Console.WriteLine(s);
        }
    }
}
