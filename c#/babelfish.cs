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

            Dictionary<string, string> dic = new Dictionary<string, string>();

            string s;
            while (!string.IsNullOrEmpty((s = Console.ReadLine())))
            {
                var p = s.Split(' ');
                dic.Add(p[1], p[0]);
            }
            while (!string.IsNullOrEmpty((s = Console.ReadLine())))
            {
                if (dic.ContainsKey(s))
                    Console.WriteLine(dic[s]);
                else
                    Console.WriteLine("eh");
            }
        }

    }
}
  
    
