using System;
using System.Collections.Generic;
using System.Linq;

namespace Kattis
{
    class Program
    {
        public static void Main(string[] args)
        {      
            string s;
            while((s = Console.ReadLine()) != null)
            {
                bool a = false, b = false;
                for (int i = 0; i < s.Length; i++)
                {
                    if (s[i] == 10 || s[i] == 13)
                        continue;
                    var toEnt = Convert.ToString(s[i], 2).PadLeft(7, '0').Reverse();
                    foreach (var k in toEnt)
                    {
                        if (k == '1')
                            a = !a;
                        else
                            b = !b;
                    }
                }
                Console.WriteLine(!a&&!b ? "free" : "trapped");
            }
        }
    }
}
