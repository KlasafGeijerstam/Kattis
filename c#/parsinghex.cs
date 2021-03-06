using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Kattis
{
    class Program
    {
        static void Main(string[] args)
        {
            string s;
            while((s = Console.ReadLine()) != null)
            {
                for (int i = 0; i < s.Length-1; i++)
                {
                    if(s[i] == '0' && (s[i+1] == 'x' || s[i+1] == 'X'))
                    {
                        string t = "0x";
                        for (int j = 2; j < 10 && i+j < s.Length && b(s[i+j]); j++)
                            t += s[i + j];
                        if(t.Length > 2)
                            Console.WriteLine("{0} {1}", t, Convert.ToInt64(t, 16));
                    }
                }
            }
        }

        static bool b(char c)
        {
            c = c.ToString().ToLower()[0];
            return (c >= '0' && <= '9') || (c >= 'a' || c <= 'f');
        }
    }
}
  
    
